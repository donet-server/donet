// DONET SOFTWARE
// Copyright (c) 2024, Donet Authors.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License version 3.
// You should have received a copy of this license along
// with this source code in a file named "LICENSE."
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program; if not, write to the Free Software Foundation,
// Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

//! Structure representing data types supported in the DC
//! language and enforcing numeric limits through constraints.

use crate::datagram::datagram::{Datagram, DatagramIterator};
use crate::dctype::*;
use crate::hashgen::DCHashGenerator;
use std::mem::size_of;

/// Numeric Range structs are used to represent a range of signed/unsigned
/// integers or floating point numbers. Used for enforcing numeric limits
/// within constraints of array, string, or blob sized types.
#[derive(Clone)]
pub struct DCNumericRange {
    range_type: DCNumberType,
    pub min: DCNumber,
    pub max: DCNumber,
}

impl DCNumericRange {
    pub fn new() -> Self {
        let mut default_min: DCNumber = DCNumber::new_floating_point(f64::NEG_INFINITY);
        let mut default_max: DCNumber = DCNumber::new_floating_point(f64::INFINITY);

        default_min.number_type = DCNumberType::None;
        default_max.number_type = DCNumberType::None;

        Self {
            range_type: DCNumberType::None,
            min: default_min,
            max: default_max,
        }
    }

    pub fn new_integer_range(min: i64, max: i64) -> Self {
        Self {
            range_type: DCNumberType::Int,
            min: DCNumber::new_integer(min),
            max: DCNumber::new_integer(max),
        }
    }

    pub fn new_unsigned_integer_range(min: u64, max: u64) -> Self {
        Self {
            range_type: DCNumberType::UInt,
            min: DCNumber::new_unsigned_integer(min),
            max: DCNumber::new_unsigned_integer(max),
        }
    }

    pub fn new_floating_point_range(min: f64, max: f64) -> Self {
        Self {
            range_type: DCNumberType::Float,
            min: DCNumber::new_floating_point(min),
            max: DCNumber::new_floating_point(max),
        }
    }

    pub fn contains(&self, num: DCNumber) -> bool {
        match self.min.number_type {
            DCNumberType::None => true,
            DCNumberType::Int => unsafe {
                /* NOTE: All reads of unions require an unsafe block due to potential UB.
                 * As the developer, we have to make sure every read of this union guarantees
                 * that it will contain the expected data type at the point we read.
                 * We make use of the DCNumberType enumerator to guarantee this safety.
                 */
                self.min.value.integer <= num.value.integer && num.value.integer <= self.max.value.integer
            },
            DCNumberType::UInt => unsafe {
                self.min.value.unsigned_integer <= num.value.unsigned_integer
                    && num.value.unsigned_integer <= self.max.value.unsigned_integer
            },
            DCNumberType::Float => unsafe {
                self.min.value.floating_point <= num.value.floating_point
                    && num.value.floating_point <= self.max.value.floating_point
            },
        }
    }

    pub fn is_empty(&self) -> bool {
        self.range_type.is_none() // using strum macro
    }
}

// ---------- Numeric Type ---------- //

pub struct DCNumericType {
    base_type: DCTypeDefinition,
    divisor: u16,
    // These are the original range and modulus values from the file, unscaled by the divisor.
    orig_modulus: f64,
    orig_range: DCNumericRange,
    // These are the range and modulus values after scaling by the divisor.
    modulus: DCNumber,
    range: DCNumericRange,
    // Specific to Donet's DC language
    explicit_cast: Option<DCTypeDefinition>,
}

pub trait DCNumericTypeInterface {
    fn new(base_type: DCTypeEnum) -> Self;
    fn generate_hash(&self, hashgen: &mut DCHashGenerator);

    fn has_modulus(&self) -> bool;
    fn has_range(&self) -> bool;

    fn get_divisor(&self) -> u16;
    fn get_modulus(&self) -> f64;
    fn get_range(&self) -> DCNumericRange;
    fn get_explicit_cast(&self) -> Option<DCTypeDefinition>;

    fn set_divisor(&mut self, divisor: u16) -> Result<(), String>;
    fn set_modulus(&mut self, modulus: f64) -> Result<(), String>;
    fn set_range(&mut self, range: DCNumericRange) -> Result<(), String>;
    fn set_explicit_cast(&mut self, dtype: DCTypeDefinition) -> Result<(), String>;

    fn within_range(&self, data: Vec<u8>, length: u64) -> Result<(), String>;
}

impl DCNumericType {
    fn data_to_number(&self, data: Vec<u8>) -> (bool, DCNumber) {
        // NOTE: See 'Deref' trait implementation for 'DCNumericType' below
        // on how we're using self.parent.size as self.size.
        if self.base_type.size != data.len().try_into().unwrap() {
            return (false, DCNumber::new_integer(0_i64));
        }

        let mut dg = Datagram::new();
        let _ = dg.add_data(data);
        let mut dgi = DatagramIterator::new(dg);

        match self.base_type.data_type {
            DCTypeEnum::TInt8 => (true, DCNumber::new_integer(i64::from(dgi.read_i8()))),
            DCTypeEnum::TInt16 => (true, DCNumber::new_integer(i64::from(dgi.read_i16()))),
            DCTypeEnum::TInt32 => (true, DCNumber::new_integer(i64::from(dgi.read_i32()))),
            DCTypeEnum::TInt64 => (true, DCNumber::new_integer(dgi.read_i64())),
            DCTypeEnum::TChar | DCTypeEnum::TUInt8 => {
                (true, DCNumber::new_unsigned_integer(u64::from(dgi.read_u8())))
            }
            DCTypeEnum::TUInt16 => (true, DCNumber::new_unsigned_integer(u64::from(dgi.read_u16()))),
            DCTypeEnum::TUInt32 => (true, DCNumber::new_unsigned_integer(u64::from(dgi.read_u32()))),
            DCTypeEnum::TUInt64 => (true, DCNumber::new_unsigned_integer(dgi.read_u64())),
            DCTypeEnum::TFloat32 => (true, DCNumber::new_floating_point(f64::from(dgi.read_f32()))),
            DCTypeEnum::TFloat64 => (true, DCNumber::new_floating_point(dgi.read_f64())),
            _ => (false, DCNumber::new_integer(0_i64)),
        }
    }
}

impl DCNumericTypeInterface for DCNumericType {
    fn new(base_type: DCTypeEnum) -> Self {
        Self {
            base_type: {
                let mut parent_struct = DCTypeDefinition::new();
                parent_struct.data_type = base_type;

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
                    _ => parent_struct.data_type = DCTypeEnum::TInvalid,
                }
                parent_struct
            },
            divisor: 1_u16,
            orig_modulus: 0.0_f64,
            orig_range: DCNumericRange::new(),
            modulus: DCNumber::new(),
            range: DCNumericRange::new(),
            explicit_cast: None,
        }
    }

    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        self.base_type.generate_hash(hashgen);
        hashgen.add_int(i32::from(self.divisor));

        if self.has_modulus() {
            // unsafe block required for accessing unions
            unsafe {
                hashgen.add_int(self.modulus.value.integer.try_into().unwrap());
            }
        }
        if self.has_range() {
            unsafe {
                hashgen.add_int(self.range.min.value.integer.try_into().unwrap());
                hashgen.add_int(self.range.max.value.integer.try_into().unwrap());
            }
        }
    }

    #[inline]
    fn has_modulus(&self) -> bool {
        self.orig_modulus != 0.0
    }
    #[inline]
    fn has_range(&self) -> bool {
        self.orig_range.is_empty()
    }
    #[inline]
    fn get_divisor(&self) -> u16 {
        self.divisor
    }
    #[inline]
    fn get_modulus(&self) -> f64 {
        self.orig_modulus
    }
    #[inline]
    fn get_range(&self) -> DCNumericRange {
        self.orig_range.clone()
    }
    #[inline]
    fn get_explicit_cast(&self) -> Option<DCTypeDefinition> {
        self.explicit_cast.clone()
    }

    fn set_divisor(&mut self, divisor: u16) -> Result<(), String> {
        if divisor == 0 {
            return Err("Cannot set the divisor to 0.".to_owned());
        }
        self.divisor = divisor;
        if self.has_range() {
            self.set_range(self.orig_range.clone())?;
        }
        if self.has_modulus() {
            self.set_modulus(self.orig_modulus)?;
        }
        Ok(())
    }

    fn set_modulus(&mut self, modulus: f64) -> Result<(), String> {
        if modulus <= 0.0_f64 {
            return Err("Modulus value cannot be less than or equal to 0.0.".to_owned());
        }
        self.orig_modulus = modulus;
        self.modulus.value.floating_point = modulus * f64::from(self.divisor);
        Ok(()) // TODO: properly validate modulus range
    }

    fn set_range(&mut self, range: DCNumericRange) -> Result<(), String> {
        self.range = range; // TODO: validate
        Ok(())
    }

    fn set_explicit_cast(&mut self, dtype: DCTypeDefinition) -> Result<(), String> {
        self.explicit_cast = Some(dtype);
        Ok(()) // TODO: do some sort of type check
    }

    fn within_range(&self, data: Vec<u8>, length: u64) -> Result<(), String> {
        todo!();
    }
}
