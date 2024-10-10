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

//! Data model for a DC Atomic Field, which represents a remote
//! procedure call method of a Distributed Class.

use crate::dcfield::DCField;
use crate::dckeyword::DCKeywordList;
use crate::dcparameter::DCParameter;
use crate::dctype::DCTypeDefinition;
use crate::hashgen::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Represents an atomic field of a Distributed Class.
/// This defines the interface to a DClass object, and is
/// always implemented as a remote procedure call (RPC).
#[derive(Debug)]
pub struct DCAtomicField<'dc> {
    base_field: DCField<'dc>,
    elements: Vec<Rc<RefCell<DCParameter<'dc>>>>,
}

impl<'dc> DCAtomicField<'dc> {
    pub(crate) fn new(name: &str, bogus_field: bool) -> Self {
        Self {
            base_field: {
                let mut new_dcfield = DCField::new(name, DCTypeDefinition::new());

                new_dcfield.set_bogus_field(bogus_field);
                new_dcfield
            },
            elements: vec![],
        }
    }

    pub fn get_num_elements(&self) -> usize {
        self.elements.len()
    }

    pub fn get_element(&self, index: usize) -> Option<Rc<RefCell<DCParameter<'dc>>>> {
        match self.elements.get(index) {
            Some(pointer) => Some(Rc::clone(pointer)), // make a new rc pointer
            None => None,
        }
    }

    pub fn set_keyword_list(&mut self, kw_list: DCKeywordList) {
        self.base_field.set_field_keyword_list(kw_list)
    }

    pub fn add_element(&mut self, element: DCParameter<'dc>) {
        self.elements.push(Rc::new(RefCell::new(element)));
    }
}

impl<'dc> DCHash for DCAtomicField<'dc> {
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        self.base_field.generate_hash(hashgen);

        hashgen.add_int(self.elements.len().try_into().unwrap());

        for param_ptr in &self.elements {
            let new_ptr: Rc<RefCell<DCParameter>> = Rc::clone(param_ptr);
            let param = new_ptr.borrow_mut();

            param.generate_hash(hashgen);
        }
    }
}

impl<'dc> std::fmt::Display for DCAtomicField<'dc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "TODO")
    }
}
