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

//! Data model for a DC Molecular field, which represents
//! a form of a field 'alias' for a collection of fields.

use crate::dcatomic::DCAtomicField;
use crate::dcfield::DCField;
use crate::hashgen::*;

/// An abstract field which provides an interface to access
/// multiple atomic fields under one field and one identifier.
#[derive(Debug)]
pub struct DCMolecularField<'dc> {
    base_field: DCField<'dc>,
    atomic_fields: Vec<DCAtomicField<'dc>>,
}

impl<'dc> std::fmt::Display for DCMolecularField<'dc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "TODO")
    }
}

impl<'dc> LegacyDCHash for DCMolecularField<'dc> {
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        self.base_field.generate_hash(hashgen);

        hashgen.add_int(self.atomic_fields.len().try_into().unwrap());

        for atomic in &self.atomic_fields {
            atomic.generate_hash(hashgen);
        }
    }
}

impl<'dc> DCMolecularField<'dc> {
    /// Adds a smart pointer to the original atomic field in our array.
    pub fn add_atomic_field(&mut self, atomic: DCAtomicField<'dc>) {
        // We should be receiving our own Rc ptr copy, so just move it into our vec.
        self.atomic_fields.push(atomic);
    }

    #[inline(always)]
    pub fn get_num_atomics(&self) -> usize {
        self.atomic_fields.len()
    }

    #[inline(always)]
    pub fn get_atomic_field(&self, index: usize) -> Option<&DCAtomicField<'dc>> {
        self.atomic_fields.get(index)
    }
}
