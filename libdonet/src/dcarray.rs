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

//! Data model of the DC Array element, which is a parameter
//! type that stores a list of values of the same data type.

use crate::dcnumeric::DCNumericRange;
use crate::dctype::{DCTypeDefinition, DCTypeEnum};
use crate::hashgen::DCHashGenerator;

pub struct DCArrayType {
    base_type: DCTypeDefinition,
    element_type: Option<DCTypeDefinition>,
    array_size: u16,
    array_range: Option<DCNumericRange>,
}

impl DCArrayType {
    pub fn new(element_type: Option<DCTypeDefinition>, size: Option<DCNumericRange>) -> Self {
        let mut new_array_type: Self = Self {
            base_type: DCTypeDefinition::new(),
            element_type,
            array_size: 0_u16,
            array_range: size,
        };

        if new_array_type.array_range.is_none() {
            new_array_type.array_range = Some(DCNumericRange::new());
            let range: &mut DCNumericRange = new_array_type.array_range.as_mut().unwrap();
            range.min.value.unsigned_integer = 0_u64;
            range.max.value.unsigned_integer = u64::MAX;
        } else {
            let range: &mut DCNumericRange = new_array_type.array_range.as_mut().unwrap();

            if range.min == range.max {
                // unsafe block required due to access of union data type
                unsafe {
                    new_array_type.array_size = range.min.value.unsigned_integer.try_into().unwrap();
                }
            }
        }

        if new_array_type.element_type.is_some() {
            let e_type: DCTypeDefinition = new_array_type.element_type.clone().unwrap();

            if !e_type.is_variable_length() && new_array_type.base_type.size > 0 {
                new_array_type.base_type.data_type = DCTypeEnum::TArray;
                new_array_type.base_type.size = new_array_type.array_size * e_type.get_size();
            } else {
                new_array_type.base_type.data_type = DCTypeEnum::TVarArray;
                new_array_type.base_type.size = 0_u16;
            }

            match e_type.get_dc_type() {
                DCTypeEnum::TChar => {
                    if new_array_type.base_type.data_type == DCTypeEnum::TArray {
                        new_array_type.base_type.data_type = DCTypeEnum::TString;
                    } else {
                        new_array_type.base_type.data_type = DCTypeEnum::TVarString;
                    }
                }
                DCTypeEnum::TUInt8 => {
                    if new_array_type.base_type.data_type == DCTypeEnum::TArray {
                        new_array_type.base_type.data_type = DCTypeEnum::TBlob;
                    } else {
                        new_array_type.base_type.data_type = DCTypeEnum::TVarBlob;
                    }
                }
                _ => {}
            }
        }
        new_array_type
    }

    /// Accumulates the properties of this DC element into the file hash.
    pub fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        self.base_type.generate_hash(hashgen);

        if let Some(element_type) = self.element_type.clone() {
            element_type.generate_hash(hashgen);
        } else {
            // Since we don't have an element type (representing
            // an 'invalid' element type, if comparing to Astron src)
            // we just make a new empty DCTypeDefinition, since that
            // is what Astron's DistributedType::invalid equals to.
            let empty_dc_type: DCTypeDefinition = DCTypeDefinition::new();
            empty_dc_type.generate_hash(hashgen);
        }
        if self.has_range() {
            // TODO!
            //hashgen.add_int(self.array_range.unwrap().min.value.integer)
        } else {
            hashgen.add_int(i32::from(self.array_size))
        }
    }

    pub fn get_array_size(&self) -> u16 {
        self.base_type.size
    }

    pub fn get_element_type(&self) -> Option<DCTypeDefinition> {
        self.element_type.clone()
    }

    pub fn get_range(&self) -> Option<DCNumericRange> {
        self.array_range.clone()
    }

    pub fn has_range(&self) -> bool {
        self.array_range.is_some()
    }
}
