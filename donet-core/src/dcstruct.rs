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

//! Data model representing a DC Struct element.
//! Very similar to a DClass element, but does not allow
//! declaring atomic or molecular fields.

use crate::dcfield::DCField;
use crate::globals;
use crate::hashgen::*;

#[derive(Debug)]
pub struct DCStruct {
    pub identifier: String,
    pub id: globals::DClassId,
    pub fields: Vec<DCField>,
    /// Whether this struct has any fields that have a constraint. (e.g. ranges)
    pub has_range: bool,
}

impl std::fmt::Display for DCStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "struct ")?;
        f.write_str(&self.identifier)?;

        write!(f, " {{  // index ")?;
        self.id.fmt(f)?;
        writeln!(f)?;

        for field in &self.fields {
            field.fmt(f)?;
        }
        writeln!(f, "}};")
    }
}

impl LegacyDCHash for DCStruct {
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        hashgen.add_string(self.identifier.clone());
        hashgen.add_int(self.fields.len().try_into().unwrap());

        for field in &self.fields {
            field.generate_hash(hashgen);
        }
    }
}
