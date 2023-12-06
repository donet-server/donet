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

use crate::dcfield::DCField;
use crate::dclass::DClass;
use crate::dcstruct::DCStruct;
use crate::globals;
use std::io::Result;
use std::sync::{Arc, Mutex};

pub struct DCImport {
    python_module: String,
    views: Vec<String>,
}

pub trait DCImportInterface {
    fn new(module_import: String, views: Vec<String>) -> DCImport;
}

impl DCImportInterface for DCImport {
    fn new(module_import: String, views: Vec<String>) -> DCImport {
        DCImport {
            python_module: module_import,
            views: views,
        }
    }
}

pub struct DCFile {
    structs: Vec<Mutex<DCStruct>>,
    dclasses: Vec<Mutex<DClass>>,
    // TODO: imports, keywords
    field_id_2_field: Vec<Arc<Mutex<DCField>>>,
    // TODO: type_id_2_type, type_name_2_type
}

#[rustfmt::skip]
pub trait DCFileInterface {
    // DC File
    fn get_hash(&mut self) -> u32;
    fn generate_hash(&mut self);
    fn add_field(&mut self, field: DCField); // assigns unique ID for the whole DC file
    // Distributed Class
    fn get_num_dclasses(&mut self) -> usize;
    fn get_dclass(&mut self, index: usize) -> Arc<Mutex<DClass>>;
    fn get_dclass_by_id(&mut self, id: globals::DClassId) -> Arc<Mutex<DClass>>;
    fn get_dclass_by_name(&mut self, name: &str) -> Arc<Mutex<DClass>>;
    fn add_dclass(&mut self, dclass: DClass) -> Result<()>;
    // DC Struct
    fn get_num_structs(&mut self) -> usize;
    fn get_struct(&mut self, index: usize) -> Arc<Mutex<DCStruct>>;
    fn add_struct(&mut self, strct: DCStruct) -> Result<()>;
}

impl DCFileInterface for DCFile {
    fn get_hash(&mut self) -> u32 {
        todo!();
    }
    fn generate_hash(&mut self) {
        todo!();
    }
    fn add_field(&mut self, field: DCField) {
        todo!();
    }
    fn get_num_dclasses(&mut self) -> usize {
        todo!();
    }
    fn get_dclass(&mut self, index: usize) -> Arc<Mutex<DClass>> {
        todo!();
    }
    fn get_dclass_by_id(&mut self, id: globals::DClassId) -> Arc<Mutex<DClass>> {
        todo!();
    }
    fn get_dclass_by_name(&mut self, name: &str) -> Arc<Mutex<DClass>> {
        todo!();
    }
    fn add_dclass(&mut self, dclass: DClass) -> Result<()> {
        todo!();
    }
    fn get_num_structs(&mut self) -> usize {
        todo!();
    }
    fn get_struct(&mut self, index: usize) -> Arc<Mutex<DCStruct>> {
        todo!();
    }
    fn add_struct(&mut self, strct: DCStruct) -> Result<()> {
        todo!();
    }
}
