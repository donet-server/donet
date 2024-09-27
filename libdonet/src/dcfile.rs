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

//! Root structure that stores the collection of DC elements
//! in memory. Provides functions for manipulating the tree.

use crate::dcfield::DCField;
use crate::dckeyword::DCKeyword;
use crate::dclass::DClass;
use crate::dcstruct::DCStruct;
use crate::globals;
use crate::hashgen::DCHashGenerator;
use crate::parser::ast;
use std::rc::Rc;

/// Data model that provides a high level representation of a single,
/// or collection, of DC files and their elements such as class imports,
/// type definitions, structures, and Distributed Classes.
#[derive(Debug)]
pub struct DCFile {
    structs: Vec<DCStruct>,
    dclasses: Vec<DClass>,
    imports: Vec<ast::PythonImport>,
    keywords: Vec<DCKeyword>,
    field_id_2_field: Vec<Rc<DCField>>,
    // TODO: type_id_2_type, type_name_2_type
    all_object_valid: bool,
    inherited_fields_stale: bool,
}

impl Default for DCFile {
    fn default() -> Self {
        Self {
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

impl DCFile {
    /// Returns a 32-bit hash index associated with this file.  This number is
    /// guaranteed to be consistent if the contents of the file have not changed,
    /// and it is very likely to be different if the contents of the file do change.
    pub fn get_hash(&self) -> globals::DCFileHash {
        let mut hashgen: DCHashGenerator = DCHashGenerator::new();

        self.generate_hash(&mut hashgen);
        hashgen.get_hash()
    }

    /// Accumulates the elements of the DC file into the hash.
    pub fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
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
            dclass.generate_hash(hashgen);
        }
    }

    /// Performs a semantic analysis on the object and its children
    /// DC elements. In Panda, this is done on the go as you build the
    /// DC file tree. Due to how we build it in memory, (and the fact
    /// that we link all the objects together until we reduce to the
    /// root production in the CFG) we have to perform this analysis
    /// until the very end when all the elements are in the DCF struct.
    pub fn semantic_analysis(&self) -> Result<(), ()> {
        // Run semantic analysis chain of all distributed class objects.
        // This should include semantic analysis for DC fields as well.
        for dclass in &self.dclasses {
            dclass.semantic_analysis()?;
        }
        // TODO!
        Ok(())
    }

    /// Returns a string with the hash as a pretty format hexadecimal.
    pub fn get_pretty_hash(&self) -> String {
        format!("0x{:0width$x}", self.get_hash(), width = 8) // 2 hex / byte = 8 hex
    }

    /// Assigns unique ID to the field for the scope of the entire DC file.
    pub fn add_field(&mut self, _field: DCField) {
        todo!();
    }

    // ---------- Python Imports ---------- //

    pub fn get_num_imports(&self) -> usize {
        self.imports.len()
    }

    pub fn get_python_import(&self, index: usize) -> ast::PythonImport {
        self.imports.get(index).unwrap().clone()
    }

    pub fn add_python_import(&mut self, import: ast::PythonImport) {
        self.imports.push(import);
    }

    // ---------- DC Keyword ---------- //

    pub fn get_num_keywords(&self) -> usize {
        todo!();
    }

    pub fn get_keyword(&self, _index: usize) -> Rc<DCKeyword> {
        todo!();
    }

    pub fn has_keyword(&self, _keyword: String) -> bool {
        todo!();
    }

    pub fn add_keyword(&mut self, _keyword: DCKeyword) {
        () // TODO!
    }

    // ---------- DC Type Definition ---------- //

    pub fn add_typedef(&mut self, _name: String) -> Result<(), ()> {
        todo!();
    }

    // ---------- Distributed Class ---------- //

    pub fn get_num_dclasses(&self) -> usize {
        self.dclasses.len()
    }

    pub fn get_next_dclass_id(&self) -> globals::DClassId {
        let dc_num: u16 = self.get_num_dclasses().try_into().unwrap();
        if dc_num == globals::DClassId::MAX {
            panic!("dcparser: Ran out of 16-bit DClass IDs!");
        }
        dc_num - 1_u16
    }

    pub fn get_dclass(&self, _index: usize) -> Rc<DClass> {
        todo!();
    }

    pub fn get_dclass_by_id(&self, _id: globals::DClassId) -> Rc<DClass> {
        todo!();
    }

    pub fn get_dclass_by_name(&self, _name: &str) -> Rc<DClass> {
        todo!();
    }

    pub fn add_dclass(&mut self, dclass: DClass) {
        self.dclasses.push(dclass);
    }

    // ---------- DC Struct ---------- //

    pub fn get_num_structs(&self) -> usize {
        todo!();
    }

    pub fn get_struct(&self, _index: usize) -> Rc<DCStruct> {
        todo!();
    }

    pub fn add_struct(&mut self, _strct: DCStruct) {
        todo!();
    }
}
