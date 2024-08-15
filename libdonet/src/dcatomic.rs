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

use crate::dcfield::DCField;
use crate::dckeyword::DCKeywordList;
use crate::dcparameter::DCParameter;
use crate::dctype::DCTypeDefinition;
use crate::hashgen::DCHashGenerator;
use std::cell::RefCell;
use std::rc::Rc;

/// Represents an atomic field of a Distributed Class.
/// This defines the interface to a DClass object, and is
/// always implemented as a remote procedure call (RPC).
#[derive(Debug)]
pub struct DCAtomicField {
    base_field: DCField,
    elements: Vec<Rc<RefCell<DCParameter>>>,
}

impl DCAtomicField {
    pub fn new(name: &str, bogus_field: bool) -> Self {
        Self {
            base_field: {
                let mut new_dcfield = DCField::new(name, DCTypeDefinition::new());

                new_dcfield.set_bogus_field(bogus_field);
                new_dcfield
            },
            elements: vec![],
        }
    }

    /// Accumulates the properties of this DC element into the file hash.
    pub fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        self.base_field.generate_hash(hashgen);

        hashgen.add_int(self.elements.len().try_into().unwrap());

        for param_ptr in &self.elements {
            let new_ptr: Rc<RefCell<DCParameter>> = Rc::clone(param_ptr);
            let param = new_ptr.borrow_mut();

            param.generate_hash(hashgen);
        }
    }

    pub fn get_num_elements(&self) -> usize {
        self.elements.len()
    }

    pub fn get_element(&self, index: usize) -> Option<Rc<RefCell<DCParameter>>> {
        match self.elements.get(index) {
            Some(pointer) => Some(Rc::clone(pointer)), // make a new rc pointer
            None => None,
        }
    }

    pub fn set_keyword_list(&mut self, kw_list: DCKeywordList) {
        self.base_field.set_field_keyword_list(kw_list)
    }

    pub fn add_element(&mut self, element: DCParameter) {
        self.elements.push(Rc::new(RefCell::new(element)));
    }
}
