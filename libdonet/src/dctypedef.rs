// DONET SOFTWARE
// Copyright (c) 2023, Donet Authors.
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

use crate::globals::DgSizeTag;
use crate::hashgen::DCHashGenerator;

#[repr(u8)] // 8-bit alignment, unsigned
#[derive(Clone)]
#[rustfmt::skip]
pub enum DCTypedefType {
    // Numeric Types
    TInt8 = 0, TInt16 = 1, TInt32 = 2, TInt64 = 3,
    TUInt8 = 4, TChar = 8, TUInt16 = 5, TUInt32 = 6, TUInt64 = 7,
    TFloat32 = 9, TFloat64 = 10,

    // Sized Data Types (Array Types)
    TString = 11, // a string with a fixed byte length
    TVarString = 12, // a string with a variable byte length
    TBlob = 13,
    TVarBlob = 14,
    TBlob32 = 19,
    TVarBlob32 = 20,
    TArray = 15,
    TVarArray = 16,

    // Complex DC Types
    TStruct = 17,
    TMethod = 18,
    TInvalid = 21,
}

pub struct DCTypeDefinition {
    alias: Option<String>,
    data_type: DCTypedefType,
    size: DgSizeTag,
}

pub trait DCTypeDefinitionInterface {
    fn new() -> DCTypeDefinition;
    fn generate_hash(&self, hashgen: &mut DCHashGenerator);

    fn get_dc_type(&self) -> DCTypedefType;
    fn is_variable_length(&self) -> bool;
    fn get_size(&self) -> DgSizeTag;

    fn has_alias(&self) -> bool;
    fn get_alias(&self) -> Result<String, ()>;
    fn set_alias(&mut self, alias: String);

    fn has_range(&self) -> bool;
    fn within_range(&self, data: Vec<u8>, length: u64) -> bool;
}

impl DCTypeDefinitionInterface for DCTypeDefinition {
    fn new() -> DCTypeDefinition {
        DCTypeDefinition {
            alias: None,
            data_type: DCTypedefType::TInvalid,
            size: 0_u16,
        }
    }

    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        hashgen.add_int(u32::from(self.data_type.clone() as u8));
        if self.alias.is_some() {
            hashgen.add_string(self.alias.clone().unwrap())
        }
    }

    fn get_dc_type(&self) -> DCTypedefType {
        self.data_type.clone()
    }

    fn is_variable_length(&self) -> bool {
        self.size == 0_u16
    }

    fn get_size(&self) -> DgSizeTag {
        self.size.clone()
    }

    fn has_alias(&self) -> bool {
        self.alias.is_some()
    }

    fn get_alias(&self) -> Result<String, ()> {
        if self.alias.is_some() {
            Ok(self.alias.clone().unwrap())
        } else {
            Err(())
        }
    }

    fn set_alias(&mut self, alias: String) {
        self.alias = Some(alias);
    }

    fn has_range(&self) -> bool {
        false
    }

    fn within_range(&self, data: Vec<u8>, length: u64) -> bool {
        todo!();
    }
}
