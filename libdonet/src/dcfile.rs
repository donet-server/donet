// DONET SOFTWARE
// Copyright (c) 2023, Donet Authors.
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

use multimap::MultiMap;

// --------- Field ---------- //

pub struct DCField<'dcfile> {
    class: Option<&'dcfile DClass<'dcfile>>,
    _struct: Option<Box<DCStruct>>,
    name: String,
    field_id: u16,
    parent_is_dclass: bool,
    default_value_stale: bool,
    has_default_value: bool,
    default_value: Vec<u8>, // stored as byte array
    bogus_field: bool,
}

pub trait DCFieldInterface<'dcfile> {
    fn new(name: &str, id: u16) -> Self;
    fn generate_hash(&mut self);
    fn set_field_id(&mut self, id: u16);
    fn set_field_name(&mut self, name: String);
    fn set_parent_struct(&mut self, parent: &'dcfile DCStruct);
    fn set_parent_dclass(&mut self, parent: &'dcfile DClass);
}

// ---------- Struct ---------- //

pub struct DCStruct {}

// ---------- DClass ---------- //

pub type FieldName2Field<'dcfile> = MultiMap<String, &'dcfile DCField<'dcfile>>;
pub type FieldIndex2Field<'dcfile> = MultiMap<u16, &'dcfile DCField<'dcfile>>;

pub struct DClass<'dcfile> {
    class_name: String,
    class_id: u16,
    is_struct: bool,
    is_bogus_class: bool,

    class_parents: Vec<&'dcfile DClass<'dcfile>>,
    constructor: Option<&'dcfile DCField<'dcfile>>,
    fields: Vec<&'dcfile DCField<'dcfile>>,
    inherited_fields: Vec<&'dcfile DCField<'dcfile>>,
    field_name_2_field: FieldName2Field<'dcfile>,
    field_index_2_field: FieldIndex2Field<'dcfile>,
}

pub trait DClassInterface<'dcfile> {
    fn new(name: &str, id: u16) -> Self;
    fn generate_hash(&mut self);

    fn set_parent(&mut self, parent: &'dcfile DClass);

    fn get_name(&mut self) -> String;
    fn get_class_id(&mut self) -> u16;
    fn get_num_parents(&mut self) -> usize;
    fn get_parent(&mut self, index: usize) -> Option<&'dcfile DClass>;
    fn has_constructor(&mut self) -> bool;
    fn get_constructor(&mut self) -> Option<&'dcfile DCField>;
}

impl<'dcfile> DClassInterface<'dcfile> for DClass<'dcfile> {
    fn new(name: &str, id: u16) -> Self {
        DClass {
            class_name: name.to_owned(),
            class_id: id,
            is_struct: false,
            is_bogus_class: true,
            class_parents: vec![],
            constructor: None,
            fields: vec![],
            inherited_fields: vec![],
            field_name_2_field: MultiMap::new(),
            field_index_2_field: MultiMap::new(),
        }
    }

    fn generate_hash(&mut self) {
        todo!(); // TODO: Implement once hash gen is written
    }

    fn set_parent(&mut self, parent: &'dcfile DClass) {
        self.class_parents.push(parent);
    }

    fn get_name(&mut self) -> String {
        self.class_name.clone()
    }

    fn get_class_id(&mut self) -> u16 {
        self.class_id
    }

    fn get_num_parents(&mut self) -> usize {
        self.class_parents.len()
    }

    fn get_parent(&mut self, index: usize) -> Option<&'dcfile DClass> {
        // copy the reference inside the option instead of a reference to the reference
        self.class_parents.get(index).copied()
    }

    fn has_constructor(&mut self) -> bool {
        self.constructor.is_some()
    }

    fn get_constructor(&mut self) -> Option<&'dcfile DCField> {
        self.constructor
    }
}
