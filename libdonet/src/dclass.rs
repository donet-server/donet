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

//! Data model for Distributed Class definitions in the DC file.
//! Stores DC Fields and tracks class hierarchy.

use crate::dcatomic::DCAtomicField;
use crate::dcfield::ClassField;
use crate::dcfile::DCFile;
use crate::globals;
use crate::hashgen::DCHashGenerator;
use multimap::MultiMap;
use std::ops::Deref;

pub type FieldName2Field<'dc> = MultiMap<String, &'dc ClassField<'dc>>;
pub type FieldId2Field<'dc> = MultiMap<globals::FieldId, &'dc ClassField<'dc>>;

/// Represents a Distributed Class defined in the DC file.
/// Contains a map of DC Fields, as well as atomic and
/// molecular fields that are declared within the class.
/// Also stores other properties such as its hierarchy.
#[derive(Debug)]
pub struct DClass<'dc> {
    dcfile: &'dc DCFile<'dc>,
    class_name: String,
    class_id: globals::DClassId,
    is_bogus_class: bool,
    class_parents: Vec<&'dc DClass<'dc>>,
    constructor: Option<&'dc DCAtomicField<'dc>>,
    fields: Vec<&'dc ClassField<'dc>>,
    inherited_fields: Vec<&'dc ClassField<'dc>>,
    field_name_2_field: FieldName2Field<'dc>,
    field_id_2_field: FieldId2Field<'dc>,
}

impl<'dc> DClass<'dc> {
    /// Accumulates the properties of this DC element into the file hash.
    pub fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        hashgen.add_string(self.get_name());
        hashgen.add_int(self.get_num_parents().try_into().unwrap());

        for parent in &self.class_parents {
            {
                hashgen.add_int(i32::from(parent.get_dclass_id()));
            }

            if let Some(constructor) = &self.constructor {
                constructor.generate_hash(hashgen);
            }
        }
        hashgen.add_int(self.fields.len().try_into().unwrap());

        for field in &self.fields {
            match &field.deref() {
                ClassField::Field(field) => field.generate_hash(hashgen),
                ClassField::Atomic(atomic) => atomic.generate_hash(hashgen),
                ClassField::Molecular(molecular) => molecular.generate_hash(hashgen),
            }
        }
    }

    pub fn get_field_by_name(&self, name: &str) -> Option<&'dc ClassField> {
        match self.field_name_2_field.get(name) {
            Some(pointer) => Some(pointer),
            None => None,
        }
    }

    #[inline(always)]
    pub fn get_name(&self) -> String {
        self.class_name.clone()
    }

    #[inline(always)]
    pub fn get_dclass_id(&self) -> globals::DClassId {
        self.class_id
    }

    #[inline(always)]
    pub fn get_num_parents(&self) -> usize {
        self.class_parents.len()
    }

    #[inline(always)]
    pub fn get_parent(&self, index: usize) -> Option<&'static DClass> {
        // copy the reference inside the option instead of a reference to the reference
        self.class_parents.get(index).cloned()
    }

    #[inline(always)]
    pub fn has_constructor(&self) -> bool {
        self.constructor.is_some()
    }

    #[inline(always)]
    pub fn get_constructor(&self) -> Option<&'dc DCAtomicField> {
        if let Some(atomic) = &self.constructor {
            Some(atomic)
        } else {
            None
        }
    }
}

pub(crate) mod intermediate {
    use crate::globals;
    use crate::parser::ast;
    use crate::parser::lexer::Span;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    pub struct DClass {
        pub span: Span,
        pub identifier: String,
        pub parents: Vec<String>,
        pub fields: ast::ClassFields,
        pub class_id: globals::DClassId,
        pub is_bogus_class: bool,
        pub class_parents: Vec<Rc<RefCell<DClass>>>,
    }

    impl DClass {
        /// Performs a semantic analysis on the object and its children.
        pub fn semantic_analysis(&self) -> Result<(), ()> {
            // TODO!
            Ok(())
        }

        #[inline(always)]
        pub fn add_parent(&mut self, parent: Rc<RefCell<DClass>>) {
            self.class_parents.push(parent);
        }

        /// Adds a newly allocated DC field to this class. The field structure
        /// in memory is moved into ownership of this class structure, and is
        /// wrapped in a [`std::cell::RefCell`] and an [`std::rc::Rc`] pointer
        ///to pass references to other elements, such as molecular fields.
        pub fn add_class_field(&mut self, field: ast::AtomicOrMolecular) {
            self.is_bogus_class = false;
            self.fields.push(field);
        }
    }
}
