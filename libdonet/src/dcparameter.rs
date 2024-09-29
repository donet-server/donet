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

//! Data model that represents a single parameter of an atomic
//! field, which together form a RPC method signature.

use crate::dcatomic::DCAtomicField;
use crate::dctype::DCTypeDefinition;
use crate::hashgen::DCHashGenerator;
use std::rc::Rc;

/// Represents the type specification of a parameter within an atomic field.
#[derive(Debug)]
pub struct DCParameter<'dc> {
    parent: Rc<DCAtomicField<'dc>>,
    base_type: DCTypeDefinition,
    identifier: String,
    type_alias: String,
    default_value: Vec<u8>,
    has_default_value: bool,
}

impl<'dc> DCParameter<'dc> {
    pub(crate) fn new(method: Rc<DCAtomicField<'dc>>, dtype: DCTypeDefinition, name: Option<&str>) -> Self {
        Self {
            parent: method,
            base_type: dtype,
            identifier: match name {
                Some(n) => n.to_owned(),
                None => String::new(),
            },
            type_alias: String::new(),
            default_value: vec![],
            has_default_value: false,
        }
    }

    /// Accumulates the properties of this DC element into the file hash.
    pub fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        self.base_type.generate_hash(hashgen);
    }

    #[inline(always)]
    pub fn get_atomic_field(&self) -> Rc<DCAtomicField<'dc>> {
        Rc::clone(&self.parent) // clone new rc pointer
    }

    #[inline(always)]
    pub fn has_default_value(&self) -> bool {
        self.has_default_value
    }

    #[inline(always)]
    pub fn get_default_value(&self) -> Vec<u8> {
        self.default_value.clone()
    }

    pub fn set_type(&mut self, dtype: DCTypeDefinition) -> Result<(), ()> {
        self.base_type = dtype;
        Ok(())
    }

    pub fn set_identifier(&mut self, name: &str) -> Result<(), ()> {
        name.clone_into(&mut self.identifier);
        Ok(())
    }

    pub fn set_default_value(&mut self, v: Vec<u8>) -> Result<(), ()> {
        self.default_value = v;
        self.has_default_value = true;
        Ok(())
    }
}
