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

//! Data model for a DC Atomic Field, which represents a remote
//! procedure call method of a Distributed Class.

use crate::dcfield::{DCField, DCFieldInterface};
use crate::dclass::DClass;
use crate::dcparameter::{DCParameter, DCParameterInterface};
use crate::dctype::{DCTypeDefinition, DCTypeDefinitionInterface};
use crate::hashgen::DCHashGenerator;
use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};

/// Represents an atomic field of a Distributed Class.
/// This defines the interface to a DClass object, and is
/// always implemented as a remote procedure call (RPC).
#[derive(Debug)]
pub struct DCAtomicField {
    base_field: DCField,
    elements: Vec<Arc<Mutex<DCParameter>>>,
}

pub trait DCAtomicFieldInterface {
    fn new(name: &str, dclass: Arc<Mutex<DClass>>, bogus_field: bool) -> Self;
    fn generate_hash(&self, hashgen: &mut DCHashGenerator);

    fn get_num_elements(&self) -> usize;
    fn get_element(&self, index: usize) -> Option<Arc<Mutex<DCParameter>>>;

    fn add_element(&mut self, element: DCParameter);
}

impl DCAtomicFieldInterface for DCAtomicField {
    fn new(name: &str, dclass: Arc<Mutex<DClass>>, bogus_field: bool) -> Self {
        Self {
            base_field: {
                let mut new_dcfield = DCField::new(name, DCTypeDefinition::new());
                new_dcfield.set_parent_dclass(dclass);
                new_dcfield.set_bogus_field(bogus_field);
                new_dcfield
            },
            elements: vec![],
        }
    }

    /// Accumulates the properties of this DC element into the file hash.
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        self.base_field.generate_hash(hashgen);

        hashgen.add_int(self.elements.len().try_into().unwrap());

        for param_ptr in &self.elements {
            let new_ptr: Arc<Mutex<DCParameter>> = param_ptr.clone();
            let mutex_ref: &Mutex<DCParameter> = new_ptr.deref();
            let param: MutexGuard<'_, DCParameter> = mutex_ref.lock().unwrap();

            param.generate_hash(hashgen);
        }
    }

    fn get_num_elements(&self) -> usize {
        self.elements.len()
    }

    fn get_element(&self, index: usize) -> Option<Arc<Mutex<DCParameter>>> {
        match self.elements.get(index) {
            Some(pointer) => Some(pointer.clone()), // make a new rc pointer
            None => None,
        }
    }

    fn add_element(&mut self, element: DCParameter) {
        self.elements.push(Arc::new(Mutex::new(element)));
    }
}
