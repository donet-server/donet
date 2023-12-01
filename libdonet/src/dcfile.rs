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

pub struct DCField {}

trait DClassInterface {
    fn new(name: &str, id: u16) -> Self;
    fn get_name(&mut self) -> String;
    fn get_class_id(&mut self) -> u16;
    fn get_num_parents(&mut self) -> usize;
    fn get_parent(&mut self, index: usize) -> Option<&Box<DClass>>;
    fn has_constructor(&mut self) -> bool;
    fn get_constructor(&mut self) -> Option<&Box<DCField>>;
}

pub type FieldName2Field = MultiMap<String, Box<DCField>>;
pub type FieldIndex2Field = MultiMap<u16, Box<DCField>>;

pub struct DClass {
    class_name: String,
    class_id: u16,
    is_struct: bool,
    is_bogus_class: bool,

    class_parents: Vec<Box<DClass>>,
    constructor: Option<Box<DCField>>,
    fields: Vec<Box<DCField>>,
    inherited_fields: Vec<Box<DCField>>,
    field_name_2_field: FieldName2Field,
    field_index_2_field: FieldIndex2Field,
}

impl DClassInterface for DClass {
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

    fn get_name(&mut self) -> String {
        self.class_name.clone()
    }

    fn get_class_id(&mut self) -> u16 {
        self.class_id
    }

    fn get_num_parents(&mut self) -> usize {
        self.class_parents.len()
    }

    fn get_parent(&mut self, index: usize) -> Option<&Box<DClass>> {
        self.class_parents.get(index)
    }

    fn has_constructor(&mut self) -> bool {
        self.constructor.is_some()
    }

    fn get_constructor(&mut self) -> Option<&Box<DCField>> {
        self.constructor.as_ref()
    }
}
