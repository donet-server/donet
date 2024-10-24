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

//! Represents all data types supported by the DC language
//! and developer-defined type alias definitions.

use crate::globals::DgSizeTag;
use crate::hashgen::*;

/// The DCTypeEnum variants have assigned u8 values
/// to keep compatibility with Astron's DC hash inputs.
#[repr(u8)] // 8-bit alignment, unsigned
#[derive(Debug, Clone, PartialEq, Eq)]
#[rustfmt::skip]
pub enum DCTypeEnum {
    // Numeric Types
    TInt8 = 0, TInt16 = 1, TInt32 = 2, TInt64 = 3,
    TUInt8 = 4, TChar = 8, TUInt16 = 5, TUInt32 = 6, TUInt64 = 7,
    TFloat32 = 9, TFloat64 = 10,

    // Sized Data Types (Array Types)
    TString = 11, // a string with a fixed byte length
    TVarString = 12, // a string with a variable byte length
    TBlob = 13, TVarBlob = 14,
    TBlob32 = 19, TVarBlob32 = 20,
    TArray = 15, TVarArray = 16,

    // Complex DC Types
    TStruct = 17, TMethod = 18,
}

impl std::fmt::Display for DCTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TInt8 => write!(f, "int8"),
            Self::TInt16 => write!(f, "int16"),
            Self::TInt32 => write!(f, "int32"),
            Self::TInt64 => write!(f, "int64"),
            Self::TUInt8 => write!(f, "uint8"),
            Self::TChar => write!(f, "char"),
            Self::TUInt16 => write!(f, "uint16"),
            Self::TUInt32 => write!(f, "uint32"),
            Self::TUInt64 => write!(f, "uint64"),
            Self::TFloat32 => write!(f, "float32"),
            Self::TFloat64 => write!(f, "float64"),
            Self::TString => write!(f, "string"),
            Self::TVarString => write!(f, "var string"),
            Self::TBlob => write!(f, "blob"),
            Self::TVarBlob => write!(f, "var blob"),
            Self::TBlob32 => write!(f, "blob32"),
            Self::TVarBlob32 => write!(f, "var blob32"),
            Self::TArray => write!(f, "array"),
            Self::TVarArray => write!(f, "var array"),
            Self::TStruct => write!(f, "struct"),
            Self::TMethod => write!(f, "method"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DCTypeDefinition {
    alias: Option<String>,
    pub data_type: DCTypeEnum,
    pub size: DgSizeTag,
}

/// Creates a new DCTypeDefinition struct with a DC type set.
impl From<DCTypeEnum> for DCTypeDefinition {
    fn from(value: DCTypeEnum) -> Self {
        Self {
            alias: None,
            data_type: value,
            size: 0_u16,
        }
    }
}

impl std::fmt::Display for DCTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "typedef ")?;
        self.data_type.fmt(f)?;
        if self.has_alias() {
            write!(f, " ")?;
            self.alias.clone().unwrap().fmt(f)?;
        }
        write!(f, ";")?;
        writeln!(f)
    }
}

impl DCHash for DCTypeDefinition {
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        hashgen.add_int(i32::from(self.data_type.clone() as u8));

        if self.alias.is_some() {
            hashgen.add_string(self.alias.clone().unwrap())
        }
    }
}

impl DCTypeDefinition {
    pub fn get_dc_type(&self) -> DCTypeEnum {
        self.data_type.clone()
    }

    #[inline(always)]
    pub fn is_variable_length(&self) -> bool {
        self.size == 0_u16
    }

    #[inline(always)]
    pub fn get_size(&self) -> DgSizeTag {
        self.size
    }

    #[inline(always)]
    pub fn has_alias(&self) -> bool {
        self.alias.is_some()
    }

    pub fn get_alias(&self) -> Result<String, ()> {
        if self.alias.is_some() {
            Ok(self.alias.clone().unwrap())
        } else {
            Err(())
        }
    }

    pub fn set_alias(&mut self, alias: String) {
        self.alias = Some(alias);
    }
}

#[derive(Copy, Clone, PartialEq)] // required for unwrapping when in an option type
pub enum DCNumber {
    Integer(i64),
    UnsignedInteger(u64),
    FloatingPoint(f64),
}

impl From<DCNumber> for i32 {
    fn from(value: DCNumber) -> i32 {
        match value {
            DCNumber::Integer(x) => x as i32,
            DCNumber::UnsignedInteger(x) => x as i32,
            DCNumber::FloatingPoint(x) => x as i32,
        }
    }
}

/// Converts a `DCNumber` to an `i64` primitive type.
///
/// Panics if `DCNumber` is not of variant `Integer`.
impl From<DCNumber> for i64 {
    fn from(value: DCNumber) -> Self {
        match value {
            DCNumber::Integer(x) => x,
            _ => panic!("DCNumber is not of variant `Integer`."),
        }
    }
}

/// Converts a `DCNumber` to an `u64` primitive type.
///
/// Panics if `DCNumber` is not of variant `UnsignedInteger`.
impl From<DCNumber> for u64 {
    fn from(value: DCNumber) -> Self {
        match value {
            DCNumber::UnsignedInteger(x) => x,
            _ => panic!("DCNumber is not of variant `UnsignedInteger`."),
        }
    }
}

/// Converts a `DCNumber` to an `f64` primitive type.
///
/// Panics if `DCNumber` is not of variant `FloatingPoint`.
impl From<DCNumber> for f64 {
    fn from(value: DCNumber) -> Self {
        match value {
            DCNumber::FloatingPoint(x) => x,
            _ => panic!("DCNumber is not of variant `FloatingPoint`."),
        }
    }
}
