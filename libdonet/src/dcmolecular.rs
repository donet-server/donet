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
use crate::dctype::DCTypeDefinition;
use crate::hashgen::*;
use std::cell::RefCell;
use std::rc::Rc;

/// An abstract field which provides an interface to access
/// multiple atomic fields under one field and one identifier.
#[derive(Debug)]
pub struct DCMolecularField<'dc> {
    base_field: DCField<'dc>,
    atomic_names: Vec<String>, // used to propagate IDs up parse tree to then assign AFs
    atomic_fields: Vec<Rc<RefCell<DCAtomicField<'dc>>>>,
}

impl<'dc> DCMolecularField<'dc> {
    pub(crate) fn new(name: &str, atomic_names: Vec<String>) -> Self {
        Self {
            base_field: DCField::new(name, DCTypeDefinition::new()),
            atomic_names,
            atomic_fields: vec![],
        }
    }

    /// Adds a smart pointer to the original atomic field in our array.
    pub fn add_atomic_field(&mut self, atomic_ptr: Rc<RefCell<DCAtomicField<'dc>>>) {
        // We should be receiving our own Rc ptr copy, so just move it into our vec.
        self.atomic_fields.push(atomic_ptr);
    }

    #[inline(always)]
    pub fn get_num_atomics(&self) -> usize {
        self.atomic_fields.len()
    }

    pub fn get_atomic_field(&self, index: usize) -> Option<Rc<RefCell<DCAtomicField<'dc>>>> {
        self.atomic_fields.get(index).cloned()
    }

    /// Used by the DC parser to get the atomic field smart pointers,
    /// once it has reached a point in the parse tree where we have
    /// the atomic fields assembled inside the DClass structure in memory.
    pub fn _get_atomic_names(&self) -> Vec<String> {
        self.atomic_names.clone()
    }

    /// Clears the temporary vector of atomic identifiers, used by the DC parser.
    pub fn _drop_atomic_names(&mut self) {
        // FIXME: We can't actually drop the damn vector, so after
        // parsing, we're left with hollow vectors in heap
        // for every damn molecular field in the DC file.
        self.atomic_names.clear();
    }
}

impl<'dc> DCHash for DCMolecularField<'dc> {
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        self.base_field.generate_hash(hashgen);

        hashgen.add_int(self.atomic_fields.len().try_into().unwrap());

        for atomic_ptr in &self.atomic_fields {
            let new_ptr: Rc<RefCell<DCAtomicField>> = Rc::clone(atomic_ptr);
            let atomic_field = new_ptr.borrow_mut();

            atomic_field.generate_hash(hashgen);
        }
    }
}
