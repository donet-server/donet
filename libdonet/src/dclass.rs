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

use crate::dcatomic::{DCAtomicField, DCAtomicFieldInterface};
use crate::dcfield::{ClassField, DCFieldInterface};
use crate::globals;
use crate::hashgen::DCHashGenerator;
use multimap::MultiMap;
use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};

pub type FieldName2Field = MultiMap<String, Arc<Mutex<ClassField>>>;
pub type FieldId2Field = MultiMap<globals::FieldId, Arc<Mutex<ClassField>>>;

#[derive(Debug)]
pub struct DClass {
    class_name: String,
    class_id: globals::DClassId,
    is_struct: bool,
    is_bogus_class: bool,
    class_parents: Vec<Arc<Mutex<DClass>>>,
    constructor: Option<Arc<Mutex<DCAtomicField>>>,
    fields: Vec<Arc<Mutex<ClassField>>>,
    inherited_fields: Vec<Arc<Mutex<ClassField>>>,
    field_name_2_field: FieldName2Field,
    field_id_2_field: FieldId2Field,
}

pub trait DClassInterface {
    fn new(name: &str) -> Self;
    fn generate_hash(&mut self, hashgen: &mut DCHashGenerator);

    fn set_parent(&mut self, parent: Arc<Mutex<DClass>>);

    fn get_name(&mut self) -> String;
    fn get_dclass_id(&mut self) -> globals::DClassId;
    fn set_dclass_id(&mut self, id: globals::DClassId);
    fn get_num_parents(&mut self) -> usize;
    fn get_parent(&mut self, index: usize) -> Option<Arc<Mutex<DClass>>>;
    fn has_constructor(&mut self) -> bool;
    fn get_constructor(&mut self) -> Option<Arc<Mutex<DCAtomicField>>>;
}

impl DClassInterface for DClass {
    fn new(name: &str) -> Self {
        DClass {
            class_name: name.to_owned(),
            class_id: 0, // assigned later
            is_struct: false,
            is_bogus_class: true,
            class_parents: vec![],
            constructor: None,
            fields: vec![],
            inherited_fields: vec![],
            field_name_2_field: MultiMap::new(),
            field_id_2_field: MultiMap::new(),
        }
    }

    fn generate_hash(&mut self, hashgen: &mut DCHashGenerator) {
        hashgen.add_string(self.get_name());
        hashgen.add_int(self.get_num_parents().try_into().unwrap());

        for parent_ptr in &self.class_parents {
            {
                let new_ptr: Arc<Mutex<DClass>> = parent_ptr.clone();
                let mut parent: MutexGuard<'_, DClass> = new_ptr.deref().lock().unwrap();

                hashgen.add_int(u32::from(parent.get_dclass_id()));
            }

            if let Some(constructor_ptr) = &self.constructor {
                let new_ptr: Arc<Mutex<DCAtomicField>> = constructor_ptr.clone();
                let atomic: MutexGuard<'_, DCAtomicField> = new_ptr.deref().lock().unwrap();

                atomic.generate_hash(hashgen);
            }
        }
        hashgen.add_int(self.fields.len().try_into().unwrap());

        for field_ptr in &self.fields {
            let new_ptr: Arc<Mutex<ClassField>> = field_ptr.clone();
            let field: MutexGuard<'_, ClassField> = new_ptr.deref().lock().unwrap();

            match &field.deref() {
                ClassField::Field(field) => field.generate_hash(hashgen),
                ClassField::Atomic(atomic) => atomic.generate_hash(hashgen),
                ClassField::Molecular(_) => todo!(),
            }
        }
    }

    fn set_parent(&mut self, parent: Arc<Mutex<DClass>>) {
        self.class_parents.push(parent);
    }

    fn get_name(&mut self) -> String {
        self.class_name.clone()
    }

    fn get_dclass_id(&mut self) -> globals::DClassId {
        self.class_id
    }

    fn set_dclass_id(&mut self, id: globals::DClassId) {
        self.class_id = id;
    }

    fn get_num_parents(&mut self) -> usize {
        self.class_parents.len()
    }

    fn get_parent(&mut self, index: usize) -> Option<Arc<Mutex<DClass>>> {
        // copy the reference inside the option instead of a reference to the reference
        self.class_parents.get(index).cloned()
    }

    fn has_constructor(&mut self) -> bool {
        self.constructor.is_some()
    }

    fn get_constructor(&mut self) -> Option<Arc<Mutex<DCAtomicField>>> {
        self.constructor.clone()
    }
}
