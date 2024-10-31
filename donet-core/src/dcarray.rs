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
use crate::dctype::{DCNumber, DCTypeDefinition, DCTypeEnum};
use crate::hashgen::*;

pub struct DCArrayType {
    base_type: Option<DCTypeDefinition>,
    element_type: Option<DCTypeDefinition>,
    array_size: u16,
    array_range: Option<DCNumericRange>,
}

impl LegacyDCHash for DCArrayType {
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        self.base_type.clone().unwrap().generate_hash(hashgen);

        if let Some(element_type) = self.element_type.clone() {
            element_type.generate_hash(hashgen);
        }
        if self.has_range() {
            hashgen.add_int(self.array_range.as_ref().unwrap().min.into())
        } else {
            hashgen.add_int(i32::from(self.array_size))
        }
    }
}

impl DCArrayType {
    pub fn new(element_type: Option<DCTypeDefinition>, size: Option<DCNumericRange>) -> Self {
        let mut new_array_type: Self = Self {
            base_type: None,
            element_type,
            array_size: 0_u16,
            array_range: size,
        };

        if new_array_type.array_range.is_none() {
            new_array_type.array_range = None;
            let range: &mut DCNumericRange = new_array_type.array_range.as_mut().unwrap();

            range.min = DCNumber::UnsignedInteger(0_u64);
            range.max = DCNumber::UnsignedInteger(u64::MAX);
        } else {
            let range: &mut DCNumericRange = new_array_type.array_range.as_mut().unwrap();

            if range.min == range.max {
                new_array_type.array_size = u64::from(range.min) as u16;
            }
        }

        if new_array_type.element_type.is_some() {
            let e_type: DCTypeDefinition = new_array_type.element_type.clone().unwrap();

            let new_base_type: &mut DCTypeDefinition = new_array_type.base_type.as_mut().unwrap();

            if !e_type.is_variable_length() && new_base_type.size > 0 {
                new_base_type.data_type = DCTypeEnum::TArray;
                new_base_type.size = new_array_type.array_size * e_type.get_size();
            } else {
                new_base_type.data_type = DCTypeEnum::TVarArray;
                new_base_type.size = 0_u16;
            }

            match e_type.get_dc_type() {
                DCTypeEnum::TChar => {
                    if new_base_type.data_type == DCTypeEnum::TArray {
                        new_base_type.data_type = DCTypeEnum::TString;
                    } else {
                        new_base_type.data_type = DCTypeEnum::TVarString;
                    }
                }
                DCTypeEnum::TUInt8 => {
                    if new_base_type.data_type == DCTypeEnum::TArray {
                        new_base_type.data_type = DCTypeEnum::TBlob;
                    } else {
                        new_base_type.data_type = DCTypeEnum::TVarBlob;
                    }
                }
                _ => {}
            }
        }
        new_array_type
    }

    #[inline(always)]
    pub fn get_array_size(&self) -> u16 {
        self.base_type.clone().unwrap().size
    }

    #[inline(always)]
    pub fn get_element_type(&self) -> Option<DCTypeDefinition> {
        self.element_type.clone()
    }

    #[inline(always)]
    pub fn get_range(&self) -> Option<DCNumericRange> {
        self.array_range.clone()
    }

    #[inline(always)]
    pub fn has_range(&self) -> bool {
        self.array_range.is_some()
    }
}
