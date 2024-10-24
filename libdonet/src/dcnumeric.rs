/*
    This file is part of Donet.

    Copyright © 2024 Max Rodriguez

    Donet is free software; you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License,
    as published by the Free Software Foundation, either version 3
    of the License, or (at your option) any later version.

    Donet is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public
    License along with Donet. If not, see <https://www.gnu.org/licenses/>.
*/

//! Structure representing data types supported in the DC
//! language and enforcing numeric limits through constraints.

use crate::datagram::datagram::Datagram;
use crate::datagram::iterator::DatagramIterator;
use crate::dctype::*;
use crate::hashgen::{DCHash, DCHashGenerator};
use std::mem::{discriminant, size_of};

/// Numeric Range structs are used to represent a range of signed/unsigned
/// integers or floating point numbers. Used for enforcing numeric limits
/// within constraints of array, string, or blob sized types.
#[derive(Clone)]
pub struct DCNumericRange {
    pub min: DCNumber,
    pub max: DCNumber,
}

impl From<std::ops::Range<i64>> for DCNumericRange {
    fn from(value: std::ops::Range<i64>) -> Self {
        Self {
            min: DCNumber::Integer(value.start),
            max: DCNumber::Integer(value.end),
        }
    }
}

impl From<std::ops::Range<u64>> for DCNumericRange {
    fn from(value: std::ops::Range<u64>) -> Self {
        Self {
            min: DCNumber::UnsignedInteger(value.start),
            max: DCNumber::UnsignedInteger(value.end),
        }
    }
}

impl From<std::ops::Range<f64>> for DCNumericRange {
    fn from(value: std::ops::Range<f64>) -> Self {
        Self {
            min: DCNumber::FloatingPoint(value.start),
            max: DCNumber::FloatingPoint(value.end),
        }
    }
}

impl DCNumericRange {
    pub fn contains(&self, num: DCNumber) -> bool {
        // Check that `num` is of the same data type as this numeric range.
        if discriminant(&self.min) == discriminant(&num) {
            return false;
        }

        match self.min {
            DCNumber::Integer(min) => {
                let num = match num {
                    DCNumber::Integer(i) => i,
                    _ => panic!("Check above makes this panic unreachable."),
                };

                let max = match self.max {
                    DCNumber::Integer(i) => i,
                    _ => panic!("Check above makes this panic unreachable."),
                };

                min <= num && num <= max
            }
            DCNumber::UnsignedInteger(min) => {
                let num = match num {
                    DCNumber::UnsignedInteger(i) => i,
                    _ => panic!("Check above makes this panic unreachable."),
                };

                let max = match self.max {
                    DCNumber::UnsignedInteger(i) => i,
                    _ => panic!("Check above makes this panic unreachable."),
                };

                min <= num && num <= max
            }
            DCNumber::FloatingPoint(min) => {
                let num = match num {
                    DCNumber::FloatingPoint(i) => i,
                    _ => panic!("Check above makes this panic unreachable."),
                };

                let max = match self.max {
                    DCNumber::FloatingPoint(i) => i,
                    _ => panic!("Check above makes this panic unreachable."),
                };

                min <= num && num <= max
            }
        }
    }
}

pub struct DCNumericType {
    base_type: DCTypeDefinition,
    divisor: u16,
    /// These are the original range and modulus values from the file, unscaled by the divisor.
    orig_modulus: f64,
    orig_range: Option<DCNumericRange>,
    /// These are the range and modulus values after scaling by the divisor.
    modulus: f64,
    range: Option<DCNumericRange>,
    /// Specific to Donet's DC language
    explicit_cast: Option<DCTypeDefinition>,
}

impl From<DCTypeEnum> for DCNumericType {
    fn from(value: DCTypeEnum) -> Self {
        Self {
            base_type: {
                let mut parent_struct = DCTypeDefinition::from(value);

                macro_rules! set_parent_size {
                    ($t:ty) => {
                        parent_struct.size = size_of::<$t>().try_into().unwrap()
                    };
                }

                match parent_struct.data_type {
                    DCTypeEnum::TChar | DCTypeEnum::TInt8 | DCTypeEnum::TUInt8 => {
                        set_parent_size!(u8)
                    }
                    DCTypeEnum::TInt16 | DCTypeEnum::TUInt16 => {
                        set_parent_size!(u16)
                    }
                    DCTypeEnum::TInt32 | DCTypeEnum::TUInt32 => {
                        set_parent_size!(u32)
                    }
                    DCTypeEnum::TInt64 | DCTypeEnum::TUInt64 => {
                        set_parent_size!(u64)
                    }
                    DCTypeEnum::TFloat32 => {
                        set_parent_size!(f32)
                    }
                    DCTypeEnum::TFloat64 => {
                        set_parent_size!(f64)
                    }
                    _ => panic!("Invalid data type!"),
                }
                parent_struct
            },
            divisor: 1_u16,
            orig_modulus: 0.0_f64,
            orig_range: None,
            modulus: 0.0_f64,
            range: None,
            explicit_cast: None,
        }
    }
}

impl DCHash for DCNumericType {
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        self.base_type.generate_hash(hashgen);

        hashgen.add_int(self.divisor.into());

        if self.has_modulus() {
            hashgen.add_int(self.modulus as i32);
        }
        if let Some(range) = &self.range {
            hashgen.add_int(range.min.into());
            hashgen.add_int(range.max.into());
        }
    }
}

impl DCNumericType {
    #[inline]
    pub fn has_modulus(&self) -> bool {
        self.orig_modulus != 0.0
    }

    #[inline]
    pub fn has_range(&self) -> bool {
        self.orig_range.is_some()
    }

    #[inline]
    pub fn get_divisor(&self) -> u16 {
        self.divisor
    }

    #[inline]
    pub fn get_modulus(&self) -> f64 {
        self.orig_modulus
    }

    #[inline]
    pub fn get_range(&self) -> Option<DCNumericRange> {
        self.orig_range.clone()
    }

    #[inline]
    pub fn get_explicit_cast(&self) -> Option<DCTypeDefinition> {
        self.explicit_cast.clone()
    }

    pub fn set_divisor(&mut self, divisor: u16) -> Result<(), String> {
        if divisor == 0 {
            return Err("Cannot set the divisor to 0.".into());
        }
        self.divisor = divisor;

        if self.has_range() {
            self.set_range(self.orig_range.clone().unwrap())?;
        }

        if self.has_modulus() {
            self.set_modulus(self.orig_modulus)?;
        }
        Ok(())
    }

    pub fn set_modulus(&mut self, modulus: f64) -> Result<(), String> {
        if modulus <= 0.0_f64 {
            return Err("Modulus value cannot be less than or equal to 0.0.".into());
        }
        self.orig_modulus = modulus;
        self.modulus = modulus * f64::from(self.divisor);

        Ok(()) // TODO: properly validate modulus range
    }

    pub fn set_range(&mut self, range: DCNumericRange) -> Result<(), String> {
        self.range = Some(range); // TODO: validate
        Ok(())
    }

    pub fn set_explicit_cast(&mut self, dtype: DCTypeDefinition) -> Result<(), String> {
        self.explicit_cast = Some(dtype);
        Ok(()) // TODO: do some sort of type check
    }

    pub fn within_range(&self, _data: Vec<u8>, _length: u64) -> Result<(), String> {
        todo!();
    }

    fn _data_to_number(&self, data: Vec<u8>) -> (bool, DCNumber) {
        if self.base_type.size != data.len().try_into().unwrap() {
            return (false, DCNumber::Integer(0_i64));
        }

        let mut dg = Datagram::default();
        dg.add_data(data).expect("Failed to convert data to datagram.");

        let mut dgi: DatagramIterator = dg.into();

        match self.base_type.data_type {
            DCTypeEnum::TInt8 => (true, DCNumber::Integer(i64::from(dgi.read_i8()))),
            DCTypeEnum::TInt16 => (true, DCNumber::Integer(i64::from(dgi.read_i16()))),
            DCTypeEnum::TInt32 => (true, DCNumber::Integer(i64::from(dgi.read_i32()))),
            DCTypeEnum::TInt64 => (true, DCNumber::Integer(dgi.read_i64())),
            DCTypeEnum::TChar | DCTypeEnum::TUInt8 => {
                (true, DCNumber::UnsignedInteger(u64::from(dgi.read_u8())))
            }
            DCTypeEnum::TUInt16 => (true, DCNumber::UnsignedInteger(u64::from(dgi.read_u16()))),
            DCTypeEnum::TUInt32 => (true, DCNumber::UnsignedInteger(u64::from(dgi.read_u32()))),
            DCTypeEnum::TUInt64 => (true, DCNumber::UnsignedInteger(dgi.read_u64())),
            DCTypeEnum::TFloat32 => (true, DCNumber::FloatingPoint(f64::from(dgi.read_f32()))),
            DCTypeEnum::TFloat64 => (true, DCNumber::FloatingPoint(dgi.read_f64())),
            _ => (false, DCNumber::Integer(0_i64)),
        }
    }
}
