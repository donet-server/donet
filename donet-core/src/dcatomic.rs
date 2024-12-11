/*
    This file is part of Donet.

    Copyright © 2024 Max Rodriguez <me@maxrdz.com>

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

//! Data model for a DC Atomic Field, which represents a remote
//! procedure call method of a Distributed Class.

use crate::dcfield::DCField;
use crate::dckeyword::DCKeywordList;
use crate::dcparameter::DCParameter;
use crate::hashgen::*;

/// Represents an atomic field of a Distributed Class.
/// This defines the interface to a DClass object, and is
/// always implemented as a remote procedure call (RPC).
#[derive(Debug)]
pub struct DCAtomicField<'dc> {
    base_field: DCField<'dc>,
    elements: Vec<&'dc DCParameter<'dc>>,
}

impl std::fmt::Display for DCAtomicField<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "TODO")
    }
}

impl LegacyDCHash for DCAtomicField<'_> {
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        self.base_field.generate_hash(hashgen);

        hashgen.add_int(self.elements.len().try_into().unwrap());

        for param in &self.elements {
            param.generate_hash(hashgen);
        }
    }
}

impl<'dc> DCAtomicField<'dc> {
    #[inline(always)]
    pub fn get_num_elements(&self) -> usize {
        self.elements.len()
    }

    #[inline(always)]
    pub fn get_element(&self, index: usize) -> Option<&'dc DCParameter<'dc>> {
        self.elements.get(index).copied()
    }

    pub fn set_keyword_list(&mut self, kw_list: DCKeywordList<'dc>) {
        self.base_field.set_field_keyword_list(kw_list)
    }
}
