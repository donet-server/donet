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
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub type FieldName2Field = MultiMap<String, Rc<RefCell<ClassField>>>;
pub type FieldId2Field = MultiMap<globals::FieldId, Rc<RefCell<ClassField>>>;

/// Represents a Distributed Class defined in the DC file.
/// Contains a map of DC Fields, as well as atomic and
/// molecular fields that are declared within the class.
/// Also stores other properties such as its hierarchy.
#[derive(Debug)]
pub struct DClass {
    dcfile: Rc<RefCell<DCFile>>, // read comment below. should reference REAL dcf by parse end.
    // FIXME: Remove this workaround code once #10 and #11 are resolved.
    dcf_assigned: bool, // due to how the parser works, we assign it 'til the end.
    class_name: String,
    class_id: globals::DClassId,
    is_bogus_class: bool,
    class_parents: Vec<Rc<RefCell<DClass>>>,
    constructor: Option<Rc<RefCell<DCAtomicField>>>,
    fields: Vec<Rc<RefCell<ClassField>>>,
    inherited_fields: Vec<Rc<RefCell<ClassField>>>,
    field_name_2_field: FieldName2Field,
    field_id_2_field: FieldId2Field,
}

impl DClass {
    pub fn new(name: &str) -> Self {
        DClass {
            dcfile: Rc::new(RefCell::new(DCFile::new())), // FIXME
            dcf_assigned: false,
            class_name: name.to_owned(),
            class_id: 0, // assigned later
            is_bogus_class: true,
            class_parents: vec![],
            constructor: None,
            fields: vec![],
            inherited_fields: vec![],
            field_name_2_field: MultiMap::new(),
            field_id_2_field: MultiMap::new(),
        }
    }

    /// Accumulates the properties of this DC element into the file hash.
    pub fn generate_hash(&mut self, hashgen: &mut DCHashGenerator) {
        hashgen.add_string(self.get_name());
        hashgen.add_int(self.get_num_parents().try_into().unwrap());

        for parent_ptr in &self.class_parents {
            {
                let mut parent = parent_ptr.borrow_mut();

                hashgen.add_int(i32::from(parent.get_dclass_id()));
            }

            if let Some(constructor) = &self.constructor {
                constructor.borrow_mut().generate_hash(hashgen);
            }
        }
        hashgen.add_int(self.fields.len().try_into().unwrap());

        for field_ptr in &self.fields {
            let field = field_ptr.borrow_mut();

            match &field.deref() {
                ClassField::Field(field) => field.generate_hash(hashgen),
                ClassField::Atomic(atomic) => atomic.generate_hash(hashgen),
                ClassField::Molecular(molecular) => molecular.generate_hash(hashgen),
            }
        }
    }

    /// Performs a semantic analysis on the object and its children.
    pub fn semantic_analysis(&self) -> Result<(), ()> {
        assert!(
            self.dcf_assigned,
            "No DC file pointer found in '{}' dclass!",
            self.class_name,
        );
        // TODO!
        Ok(())
    }

    pub fn set_dcfile(&mut self, dcf: Rc<RefCell<DCFile>>) {
        assert!(
            !self.dcf_assigned,
            "Tried to reassign DC file pointer to '{}' class",
            self.class_name
        );
        self.dcfile = dcf;
        self.dcf_assigned = true;
    }

    #[inline(always)]
    pub fn add_parent(&mut self, parent: Rc<RefCell<DClass>>) {
        self.class_parents.push(parent);
    }

    /// Adds a newly allocated DC field to this class. The field structure
    /// in memory is moved into ownership of this class structure, and is
    /// wrapped in a [`std::cell::RefCell`] and an [`std::rc::Rc`] pointer
    ///to pass references to other elements, such as molecular fields.
    pub fn add_class_field(&mut self, field: ClassField) {
        self.is_bogus_class = false;
        self.fields.push(Rc::new(RefCell::new(field)));
    }

    pub fn get_field_by_name(&mut self, name: &str) -> Option<Rc<RefCell<ClassField>>> {
        match self.field_name_2_field.get(name) {
            Some(pointer) => Some(Rc::clone(&pointer)),
            None => None,
        }
    }

    #[inline(always)]
    pub fn get_name(&mut self) -> String {
        self.class_name.clone()
    }

    #[inline(always)]
    pub fn get_dclass_id(&mut self) -> globals::DClassId {
        self.class_id
    }

    #[inline(always)]
    pub fn set_dclass_id(&mut self, id: globals::DClassId) {
        self.class_id = id;
    }

    #[inline(always)]
    pub fn get_num_parents(&mut self) -> usize {
        self.class_parents.len()
    }

    #[inline(always)]
    pub fn get_parent(&mut self, index: usize) -> Option<Rc<RefCell<DClass>>> {
        // copy the reference inside the option instead of a reference to the reference
        self.class_parents.get(index).cloned()
    }

    #[inline(always)]
    pub fn has_constructor(&mut self) -> bool {
        self.constructor.is_some()
    }

    #[inline(always)]
    pub fn get_constructor(&mut self) -> Option<Rc<RefCell<DCAtomicField>>> {
        if let Some(atomic) = &self.constructor {
            Some(Rc::clone(atomic))
        } else {
            None
        }
    }
}
