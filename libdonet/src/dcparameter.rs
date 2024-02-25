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
use crate::dctype::{DCTypeDefinition, DCTypeDefinitionInterface};
use crate::hashgen::DCHashGenerator;
use std::sync::Arc;

/// Represents the type specification of a parameter within an atomic field.
#[derive(Debug)]
pub struct DCParameter {
    parent: Arc<DCAtomicField>,
    base_type: DCTypeDefinition,
    name: String,
    type_alias: String,
    default_value: Vec<u8>,
    has_default_value: bool,
}

pub trait DCParameterInterface {
    fn new(method: Arc<DCAtomicField>, dtype: DCTypeDefinition, name: Option<&str>) -> Self;
    fn generate_hash(&self, hashgen: &mut DCHashGenerator);

    fn get_atomic_field(&self) -> Arc<DCAtomicField>;
    fn has_default_value(&self) -> bool;
    fn get_default_value(&self) -> Vec<u8>;

    fn set_type(&mut self, dtype: DCTypeDefinition) -> Result<(), ()>;
    fn set_name(&mut self, name: &str) -> Result<(), ()>;
    fn set_default_value(&mut self, v: Vec<u8>) -> Result<(), ()>;
}

impl DCParameterInterface for DCParameter {
    fn new(method: Arc<DCAtomicField>, dtype: DCTypeDefinition, name: Option<&str>) -> Self {
        Self {
            parent: method,
            base_type: dtype,
            name: match name {
                Some(n) => n.to_owned(),
                None => String::new(),
            },
            type_alias: String::new(),
            default_value: vec![],
            has_default_value: false,
        }
    }

    /// Accumulates the properties of this DC element into the file hash.
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        self.base_type.generate_hash(hashgen);
    }

    fn get_atomic_field(&self) -> Arc<DCAtomicField> {
        self.parent.clone() // clone new arc pointer
    }

    #[inline(always)]
    fn has_default_value(&self) -> bool {
        self.has_default_value
    }

    fn get_default_value(&self) -> Vec<u8> {
        self.default_value.clone()
    }

    fn set_type(&mut self, dtype: DCTypeDefinition) -> Result<(), ()> {
        self.base_type = dtype;
        Ok(())
    }

    fn set_name(&mut self, name: &str) -> Result<(), ()> {
        self.name = name.to_owned();
        Ok(())
    }

    fn set_default_value(&mut self, v: Vec<u8>) -> Result<(), ()> {
        self.default_value = v;
        self.has_default_value = true;
        Ok(())
    }
}
