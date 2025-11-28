/*
    This file is part of Donet.

    Copyright © 2024-2025 Max Rodriguez <me@maxrdz.com>

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

//! Data model that represents a single parameter of an atomic
//! field, which together form a RPC method signature.

use crate::dctype::DCTypeDefinition;
use crate::hashgen::*;

/// Represents the type specification of a parameter,
/// which can live under an atomic field, or become
/// a standalone parameter field. (e.g. for structs)
#[derive(Debug)]
pub struct DCParameter {
    base_type: DCTypeDefinition,
    identifier: Option<String>,
    default_value: Vec<u8>,
}

impl std::fmt::Display for DCParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.base_type.data_type.fmt(f)?;

        // if we have an identifier, write it out
        if let Some(id) = &self.identifier {
            write!(f, " {}", id)?;
        }
        // if we have a default value, write it as a hex literal
        if self.has_default_value() {
            write!(
                f,
                " = 0x{}",
                self.default_value
                    .clone()
                    .iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<String>>()
                    .join("")
            )?;
        }
        Ok(())
    }
}

impl LegacyDCHash for DCParameter {
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        self.base_type.generate_hash(hashgen);
    }
}

impl DCParameter {
    #[inline(always)]
    pub fn has_identifier(&self) -> bool {
        self.identifier.is_some()
    }

    #[inline(always)]
    pub fn has_default_value(&self) -> bool {
        !self.default_value.is_empty()
    }

    #[inline(always)]
    pub fn get_default_value(&self) -> &Vec<u8> {
        &self.default_value
    }
}
