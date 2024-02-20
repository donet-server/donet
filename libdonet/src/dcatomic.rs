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

use crate::dcfield::{DCField, DCFieldInterface};
use crate::dclass::DClass;
use crate::dcparameter::DCParameterField;
use crate::dctype::{DCTypeDefinition, DCTypeDefinitionInterface};
use crate::hashgen::DCHashGenerator;
use std::sync::{Arc, Mutex};

/// Represents an atomic field of a Distributed Class.
/// This defines the interface to a DClass object, and is
/// always implemented as a remote procedure call (RPC).
pub struct DCAtomicField {
    _dcatomicfield_parent: DCField,
    elements: Vec<Arc<Mutex<DCParameterField>>>,
}

pub trait DCAtomicFieldInterface {
    fn new(name: &str, dclass: Arc<Mutex<DClass>>, bogus_field: bool) -> Self;
    fn generate_hash(&self, hashgen: &mut DCHashGenerator);

    fn get_num_elements(&self) -> usize;
    fn get_element(&self, index: usize) -> Option<Arc<Mutex<DCParameterField>>>;

    fn add_element(&mut self, element: DCParameterField);
}

impl DCAtomicFieldInterface for DCAtomicField {
    fn new(name: &str, dclass: Arc<Mutex<DClass>>, bogus_field: bool) -> Self {
        Self {
            _dcatomicfield_parent: {
                let mut new_dcfield = DCField::new(name, DCTypeDefinition::new());
                new_dcfield.set_parent_dclass(dclass);
                new_dcfield.set_bogus_field(bogus_field);
                new_dcfield
            },
            elements: vec![],
        }
    }

    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        self._dcatomicfield_parent.dcfield_generate_hash(hashgen);
        // TODO!
    }

    fn get_num_elements(&self) -> usize {
        self.elements.len()
    }

    fn get_element(&self, index: usize) -> Option<Arc<Mutex<DCParameterField>>> {
        match self.elements.get(index) {
            Some(pointer) => Some(pointer.clone()), // make a new rc pointer
            None => None,
        }
    }

    fn add_element(&mut self, element: DCParameterField) {
        self.elements.push(Arc::new(Mutex::new(element)));
    }
}

/// See issue #22.
impl std::ops::Deref for DCAtomicField {
    type Target = DCField;
    fn deref(&self) -> &Self::Target {
        &self._dcatomicfield_parent
    }
}
