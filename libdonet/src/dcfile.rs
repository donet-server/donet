/*
    This file is part of Donet.

    Copyright © 2024 Max Rodriguez

    Donet is free software; you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License,
    as published by the Free Software Foundation, either version 3
    of the License, or (at your option) any later version.

    Donet is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public
    License along with Donet. If not, see <https://www.gnu.org/licenses/>.
*/

//! Root structure that stores the collection of DC elements
//! in memory. Provides functions for manipulating the tree.

use crate::dcfield::DCField;
use crate::dckeyword::DCKeyword;
use crate::dclass::DClass;
use crate::dcstruct::DCStruct;
use crate::dctype::DCTypeDefinition;
use crate::globals;
use crate::hashgen::*;
use crate::parser::ast;

/// Data model that provides a high level representation of a single,
/// or collection, of DC files and their elements such as class imports,
/// type definitions, structures, and Distributed Classes.
pub struct DCFile<'dc> {
    baked_hash: globals::DCFileHash,
    structs: Vec<DCStruct>,
    dclasses: Vec<DClass<'dc>>,
    imports: Vec<ast::PyModuleImport>,
    keywords: Vec<DCKeyword>,
    type_defs: Vec<DCTypeDefinition>,
    field_id_2_field: Vec<&'dc DCField<'dc>>,
    // TODO: type_id_2_type, type_name_2_type
    all_object_valid: bool,
    inherited_fields_stale: bool,
}

impl<'dc> DCFile<'dc> {
    pub(crate) fn new(
        structs: Vec<DCStruct>,
        dclasses: Vec<DClass<'dc>>,
        imports: Vec<ast::PyModuleImport>,
        keywords: Vec<DCKeyword>,
        type_defs: Vec<DCTypeDefinition>,
        field_id_2_field: Vec<&'dc DCField<'dc>>,
        all_object_valid: bool,
        inherited_fields_stale: bool,
    ) -> Self {
        Self {
            baked_hash: 0_u32,
            structs,
            dclasses,
            imports,
            keywords,
            type_defs,
            field_id_2_field,
            all_object_valid,
            inherited_fields_stale,
        }
    }

    /// Returns a 32-bit hash index associated with this file.  This number is
    /// guaranteed to be consistent if the contents of the file have not changed,
    /// and it is very likely to be different if the contents of the file do change.
    ///
    /// If called more than once, it will reuse the already calculated hash,
    /// as this structure is guaranteed to be immutable after initialization.
    pub fn get_hash(&self) -> globals::DCFileHash {
        if self.baked_hash != 0 {
            self.baked_hash
        } else {
            let mut hashgen: DCHashGenerator = DCHashGenerator::default();

            self.generate_hash(&mut hashgen);
            hashgen.get_hash()
        }
    }

    /// Returns a string with the hash as a pretty format hexadecimal.
    pub fn get_pretty_hash(&self) -> String {
        format!("0x{:0width$x}", self.get_hash(), width = 8) // 2 hex / byte = 8 hex
    }

    // ---------- Python Imports ---------- //

    pub fn get_num_imports(&self) -> usize {
        self.imports.len()
    }

    pub fn get_python_import(&self, index: usize) -> ast::PyModuleImport {
        self.imports.get(index).unwrap().clone()
    }

    // ---------- DC Keyword ---------- //

    pub fn get_num_keywords(&self) -> usize {
        todo!();
    }

    pub fn get_keyword(&self, _index: usize) -> &'dc DCKeyword {
        todo!();
    }

    pub fn has_keyword(&self, _keyword: String) -> bool {
        todo!();
    }

    // ---------- Distributed Class ---------- //

    pub fn get_num_dclasses(&self) -> usize {
        self.dclasses.len()
    }

    pub fn get_dclass(&self, _index: usize) -> &'dc DClass {
        todo!();
    }

    pub fn get_dclass_by_id(&self, id: globals::DClassId) -> &'dc DClass {
        self.dclasses.get(usize::from(id)).unwrap()
    }

    pub fn get_dclass_by_name(&self, _name: &str) -> &'dc DClass {
        todo!();
    }

    // ---------- DC Struct ---------- //

    pub fn get_num_structs(&self) -> usize {
        todo!();
    }

    pub fn get_struct(&self, _index: usize) -> &'dc DCStruct {
        todo!();
    }
}

impl<'dc> DCHash for DCFile<'dc> {
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        if globals::DC_VIRTUAL_INHERITANCE {
            // Just to change the hash output in this case.
            if globals::DC_SORT_INHERITANCE_BY_FILE {
                hashgen.add_int(1);
            } else {
                hashgen.add_int(2);
            }
        }
        hashgen.add_int(self.get_num_dclasses().try_into().unwrap());

        for dclass in &self.dclasses {
            dclass.generate_hash(hashgen);
        }
    }
}

impl<'dc> std::fmt::Debug for DCFile<'dc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Print Python-style imports
        if !self.imports.is_empty() {
            for import in &self.imports {
                if import.symbols.is_empty() {
                    write!(f, "import ")?;
                    f.write_str(&import.python_module)?;
                    writeln!(f)?;
                } else {
                    write!(f, "from ")?;
                    f.write_str(&import.python_module)?;

                    write!(f, "import ")?;
                    for (i, symbol) in import.symbols.iter().enumerate() {
                        f.write_str(symbol)?;

                        if i != import.symbols.len() - 1 {
                            write!(f, ", ")?;
                        }
                    }
                    writeln!(f)?;
                }
            }
            writeln!(f)?;
        }
        // Print type declarations
        for type_def in &self.type_defs {
            type_def.fmt(f)?;
            writeln!(f)?;
        }
        for kw in &self.keywords {
            kw.fmt(f)?;
            writeln!(f)?;
        }
        for strukt in &self.structs {
            strukt.fmt(f)?;
            writeln!(f)?;
        }
        for dclass in &self.dclasses {
            dclass.fmt(f)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

pub(crate) mod intermediate {
    use super::*;
    use crate::dclass::intermediate::DClass;

    /// DC file structure for internal use by the DC parser.
    #[derive(Debug)]
    pub struct DCFile {
        pub structs: Vec<DCStruct>,
        pub dclasses: Vec<DClass>,
        pub imports: Vec<ast::PythonImport>,
        pub keywords: Vec<DCKeyword>,
        //pub field_id_2_field: Vec<Rc<DCField>>,
        // TODO: type_id_2_type, type_name_2_type
        pub all_object_valid: bool,
        pub inherited_fields_stale: bool,
    }

    impl Default for DCFile {
        fn default() -> Self {
            Self {
                structs: vec![],
                dclasses: vec![],
                imports: vec![],
                keywords: vec![],
                //field_id_2_field: vec![],
                all_object_valid: true,
                inherited_fields_stale: false,
            }
        }
    }

    impl DCFile {
        /// Performs a semantic analysis on the object and its children
        /// DC elements. In Panda, this is done on the go as you build the
        /// DC file tree. Due to how we build it in memory, (and the fact
        /// that we link all the objects together until we reduce to the
        /// root production in the CFG) we have to perform this analysis
        /// until the very end when all the elements are in the DCF struct.
        pub fn semantic_analysis(&self) -> Result<(), ()> {
            // Run semantic analysis chain of all distributed class objects.
            // This should include semantic analysis for DC fields as well.
            //for dclass in &self.dclasses {
            //dclass.semantic_analysis()?;
            //}
            // TODO!
            Ok(())
        }

        /// Assigns unique ID to the field for the scope of the entire DC file.
        pub fn add_field(&mut self, _field: DCField) {
            todo!();
        }

        pub fn add_python_import(&mut self, import: ast::PythonImport) {
            self.imports.push(import);
        }

        pub fn add_keyword(&mut self, _keyword: DCKeyword) {
            () // TODO!
        }

        pub fn add_typedef(&mut self, _name: String) -> Result<(), ()> {
            todo!();
        }

        pub fn add_dclass(&mut self, dclass: DClass) {
            self.dclasses.push(dclass);
        }

        pub fn get_num_dclasses(&mut self) -> usize {
            self.dclasses.len()
        }

        pub fn get_next_dclass_id(&mut self) -> globals::DClassId {
            let dc_num: u16 = self.get_num_dclasses().try_into().unwrap();
            if dc_num == globals::DClassId::MAX {
                panic!("dcparser: Ran out of 16-bit DClass IDs!");
            }
            dc_num - 1_u16
        }

        pub fn add_struct(&mut self, _strct: DCStruct) {
            todo!();
        }
    }
}
