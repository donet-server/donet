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

use crate::dckeyword::DCKeyword;
use crate::dclass::DClass;
use crate::dcstruct::DCStruct;
use crate::globals;
use crate::hashgen::DCHashGenerator;
use crate::{dcfield::DCField, dclass::DClassInterface};
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Debug, Clone)]
pub struct DCImport {
    pub python_module: String,
    pub symbols: Vec<String>,
}

impl DCImport {
    pub fn new(mod_: String, symbols: Vec<String>) -> DCImport {
        DCImport {
            python_module: mod_,
            symbols: symbols,
        }
    }
}

#[derive(Debug)]
pub struct DCFile {
    structs: Vec<Mutex<DCStruct>>,
    dclasses: Vec<Mutex<DClass>>,
    imports: Vec<DCImport>, // not modified after declaration; no mutex required.
    keywords: Vec<DCKeyword>,
    field_id_2_field: Vec<Arc<Mutex<DCField>>>,
    // TODO: type_id_2_type, type_name_2_type
    all_object_valid: bool,
    inherited_fields_stale: bool,
}

#[rustfmt::skip]
pub trait DCFileInterface {
    fn get_hash(&mut self) -> globals::DCFileHash;
    fn generate_hash(&mut self, hashgen: &mut DCHashGenerator);
    fn get_pretty_hash(&mut self) -> String;
    fn add_field(&mut self, field: DCField); // assigns unique ID for the whole DC file

    // Python Imports
    fn get_num_imports(&mut self) -> usize;
    fn get_python_import(&mut self, index: usize) -> DCImport;
    fn add_python_import(&mut self, import: DCImport);

    // DC Keyword
    fn get_num_keywords(&self) -> usize;
    fn get_keyword(&self, index: usize) -> Arc<DCKeyword>;
    fn has_keyword(&self, keyword: String) -> bool;
    fn add_keyword(&mut self, keyword: DCKeyword);

    // Type Definition
    fn add_typedef(&mut self, name: String) -> Result<(), ()>;

    // Distributed Class
    fn get_num_dclasses(&mut self) -> usize;
    fn get_next_dclass_id(&mut self) -> globals::DClassId;
    fn get_dclass(&mut self, index: usize) -> Arc<Mutex<DClass>>;
    fn get_dclass_by_id(&mut self, id: globals::DClassId) -> Arc<Mutex<DClass>>;
    fn get_dclass_by_name(&mut self, name: &str) -> Arc<Mutex<DClass>>;
    fn add_dclass(&mut self, dclass: DClass);

    // DC Struct
    fn get_num_structs(&mut self) -> usize;
    fn get_struct(&mut self, index: usize) -> Arc<Mutex<DCStruct>>;
    fn add_struct(&mut self, strct: DCStruct);
}

impl DCFile {
    pub fn new() -> DCFile {
        DCFile {
            structs: vec![],
            dclasses: vec![],
            imports: vec![],
            keywords: vec![],
            field_id_2_field: vec![],
            all_object_valid: true,
            inherited_fields_stale: false,
        }
    }
}

impl DCFileInterface for DCFile {
    /* Returns a 32-bit hash index associated with this file.  This number is
     * guaranteed to be consistent if the contents of the file have not changed,
     * and it is very likely to be different if the contents of the file do change.
     */
    fn get_hash(&mut self) -> globals::DCFileHash {
        let mut hashgen: DCHashGenerator = DCHashGenerator::new();
        self.generate_hash(&mut hashgen);
        hashgen.get_hash()
    }

    // Accumulates the elements of the DC file into the hash.
    fn generate_hash(&mut self, hashgen: &mut DCHashGenerator) {
        if globals::DC_VIRTUAL_INHERITANCE {
            // Just to change the hash output in this case.
            if globals::DC_SORT_INHERITANCE_BY_FILE {
                hashgen.add_int(1_i32);
            } else {
                hashgen.add_int(2_i32);
            }
        }
        hashgen.add_int(self.get_num_dclasses().try_into().unwrap());

        for dclass in &self.dclasses {
            let mut locked_dclass: MutexGuard<'_, DClass> = dclass.lock().unwrap();
            locked_dclass.generate_hash(hashgen);
        }
    }

    // Returns a string with the hash as a pretty format hexadecimal.
    fn get_pretty_hash(&mut self) -> String {
        format!("0x{:0width$x}", self.get_hash(), width = 8) // 2 hex / byte = 8 hex
    }

    fn add_field(&mut self, field: DCField) {
        todo!();
    }

    // ---------- Python Imports ---------- //

    fn get_num_imports(&mut self) -> usize {
        self.imports.len()
    }

    fn get_python_import(&mut self, index: usize) -> DCImport {
        self.imports.get(index).unwrap().clone()
    }

    fn add_python_import(&mut self, import: DCImport) {
        self.imports.push(import);
    }

    // ---------- DC Keyword ---------- //

    fn get_num_keywords(&self) -> usize {
        todo!();
    }

    fn get_keyword(&self, index: usize) -> Arc<DCKeyword> {
        todo!();
    }

    fn has_keyword(&self, keyword: String) -> bool {
        todo!();
    }

    fn add_keyword(&mut self, keyword: DCKeyword) {
        () // TODO!
    }

    // ---------- DC Type Definition ---------- //

    fn add_typedef(&mut self, name: String) -> Result<(), ()> {
        todo!();
    }

    // ---------- Distributed Class ---------- //

    fn get_num_dclasses(&mut self) -> usize {
        self.dclasses.len()
    }

    fn get_next_dclass_id(&mut self) -> globals::DClassId {
        let dc_num: u16 = self.get_num_dclasses().try_into().unwrap();
        if dc_num == globals::DClassId::MAX {
            panic!("dcparser: Ran out of 16-bit DClass IDs!");
        }
        dc_num - 1_u16
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

    fn add_dclass(&mut self, dclass: DClass) {
        self.dclasses.push(Mutex::new(dclass));
    }

    // ---------- DC Struct ---------- //

    fn get_num_structs(&mut self) -> usize {
        todo!();
    }

    fn get_struct(&mut self, index: usize) -> Arc<Mutex<DCStruct>> {
        todo!();
    }

    fn add_struct(&mut self, strct: DCStruct) {
        todo!();
    }
}
