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

//! Data model for a DC Molecular field, which represents
//! a form of a field 'alias' for a collection of fields.

use crate::dcatomic::{DCAtomicField, DCAtomicFieldInterface};
use crate::dcfield::{DCField, DCFieldInterface};
use crate::dctype::{DCTypeDefinition, DCTypeDefinitionInterface};
use crate::hashgen::DCHashGenerator;
use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};

/// An abstract field which provides an interface to access
/// multiple atomic fields under one field and one identifier.
#[derive(Debug)]
pub struct DCMolecularField {
    base_field: DCField,
    atomic_names: Vec<String>, // used to propagate IDs up parse tree to then assign AFs
    atomic_fields: Vec<Arc<Mutex<DCAtomicField>>>,
}

pub trait DCMolecularFieldInterface {
    fn new(name: &str, atomic_names: Vec<String>) -> Self;
    fn generate_hash(&self, hashgen: &mut DCHashGenerator);

    fn add_atomic_field(&mut self, atomic_ptr: Arc<Mutex<DCAtomicField>>);

    fn get_num_atomics(&self) -> usize;
    fn get_atomic_field(&self, index: usize) -> Option<Arc<Mutex<DCAtomicField>>>;

    fn _get_atomic_names(&self) -> Vec<String>;
    fn _drop_atomic_names(&mut self);
}

impl DCMolecularFieldInterface for DCMolecularField {
    fn new(name: &str, atomic_names: Vec<String>) -> Self {
        Self {
            base_field: {
                let mut new_field = DCField::new(name, DCTypeDefinition::new());
                new_field
            },
            atomic_names: atomic_names,
            atomic_fields: vec![],
        }
    }

    /// Accumulates the properties of this DC element into the file hash.
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        self.base_field.generate_hash(hashgen);

        hashgen.add_int(self.atomic_fields.len().try_into().unwrap());

        for atomic_ptr in &self.atomic_fields {
            let new_ptr: Arc<Mutex<DCAtomicField>> = atomic_ptr.clone();
            let mutex_ref: &Mutex<DCAtomicField> = new_ptr.deref();
            let atomic_field: MutexGuard<'_, DCAtomicField> = mutex_ref.lock().unwrap();

            atomic_field.generate_hash(hashgen);
        }
    }

    /// Adds a smart pointer to the original atomic field in our array.
    fn add_atomic_field(&mut self, atomic_ptr: Arc<Mutex<DCAtomicField>>) {
        // We should be receiving our own Arc ptr copy, so just move it into our vec.
        self.atomic_fields.push(atomic_ptr);
    }

    #[inline(always)]
    fn get_num_atomics(&self) -> usize {
        self.atomic_fields.len()
    }

    fn get_atomic_field(&self, index: usize) -> Option<Arc<Mutex<DCAtomicField>>> {
        self.atomic_fields.get(index).cloned()
    }

    /// Used by the DC parser to get the atomic field smart pointers,
    /// once it has reached a point in the parse tree where we have
    /// the atomic fields assembled inside the DClass structure in memory.
    fn _get_atomic_names(&self) -> Vec<String> {
        self.atomic_names.clone()
    }

    /// Clears the temporary vector of atomic identifiers, used by the DC parser.
    fn _drop_atomic_names(&mut self) {
        // FIXME: We can't actually drop the damn vector, so after
        // parsing, we're left with hollow vectors in heap
        // for every damn molecular field in the DC file.
        self.atomic_names.clear();
    }
}
