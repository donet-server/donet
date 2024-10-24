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

/// Represents a Python-style import statement in the DC file.
#[derive(Debug)]
pub struct DCPythonImport {
    pub module: String,
    pub symbols: Vec<String>,
}

impl From<interim::PythonImport> for DCPythonImport {
    fn from(value: interim::PythonImport) -> Self {
        Self {
            module: value.module,
            symbols: value.symbols,
        }
    }
}

impl std::fmt::Display for DCPythonImport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.symbols.is_empty() {
            write!(f, "import ")?;
            f.write_str(&self.module)?;
        } else {
            write!(f, "from ")?;
            f.write_str(&self.module)?;

            write!(f, " import ")?;
            for (i, symbol) in self.symbols.iter().enumerate() {
                f.write_str(symbol)?;

                if i != self.symbols.len() - 1 {
                    write!(f, ", ")?;
                }
            }
        }
        Ok(())
    }
}

/// Data model that provides a high level representation of a single,
/// or collection, of DC files and their elements such as class imports,
/// type definitions, structures, and Distributed Classes.
#[derive(Debug)]
pub struct DCFile<'dc> {
    baked_hash: globals::DCFileHash,
    structs: Vec<DCStruct>,
    dclasses: Vec<DClass<'dc>>,
    imports: Vec<DCPythonImport>,
    keywords: Vec<DCKeyword>,
    type_defs: Vec<DCTypeDefinition>,
    field_id_2_field: Vec<&'dc DCField<'dc>>,
    // TODO: type_id_2_type, type_name_2_type
    all_object_valid: bool,
    inherited_fields_stale: bool,
}

impl<'dc> From<interim::DCFile> for DCFile<'dc> {
    fn from(value: interim::DCFile) -> Self {
        let mut imports: Vec<DCPythonImport> = vec![];

        for imp in value.imports {
            imports.push(imp.into());
        }

        Self {
            baked_hash: 0_u32,
            structs: vec![],
            dclasses: vec![],
            imports,
            keywords: vec![],
            type_defs: vec![],
            field_id_2_field: vec![],
            all_object_valid: true,
            inherited_fields_stale: false,
        }
    }
}

impl<'dc> std::fmt::Display for DCFile<'dc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Print Python-style imports
        if !self.imports.is_empty() {
            for import in &self.imports {
                import.fmt(f)?;
                writeln!(f)?;
            }
            writeln!(f)?;
        }
        // Print type definitions
        for type_def in &self.type_defs {
            type_def.fmt(f)?;
            writeln!(f)?;
        }
        // Print Keyword definitions
        for kw in &self.keywords {
            kw.fmt(f)?;
            writeln!(f)?;
        }
        // Print Structs
        for strukt in &self.structs {
            strukt.fmt(f)?;
            writeln!(f)?;
        }
        // Print DClasses
        for dclass in &self.dclasses {
            dclass.fmt(f)?;
            writeln!(f)?;
        }
        Ok(())
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

        for strukt in &self.structs {
            strukt.generate_hash(hashgen);
        }

        for dclass in &self.dclasses {
            dclass.generate_hash(hashgen);
        }
    }
}

impl<'dc> DCFile<'dc> {
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

    pub fn get_python_import(&self, index: usize) -> &DCPythonImport {
        self.imports.get(index).expect("Index out of bounds.")
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

#[cfg(test)]
mod unit_testing {
    use super::*;

    #[test]
    fn write_dc_python_import() {
        let import: DCPythonImport = DCPythonImport {
            module: "views".to_string(),
            symbols: vec![],
        };

        assert_eq!(import.to_string(), "import views");
    }

    #[test]
    fn write_dcfile_py_imports() {
        let imports: Vec<DCPythonImport> = vec![
            DCPythonImport {
                module: "views".to_string(),
                symbols: vec![],
            },
            DCPythonImport {
                module: "views".to_string(),
                symbols: vec!["DistributedDonut".to_string()],
            },
            DCPythonImport {
                module: "views".to_string(),
                symbols: vec!["Class".to_string(), "ClassAI".to_string(), "ClassOV".to_string()],
            },
        ];

        let dcf: DCFile<'_> = DCFile {
            baked_hash: 0_u32,
            structs: vec![],
            dclasses: vec![],
            imports,
            keywords: vec![],
            type_defs: vec![],
            field_id_2_field: vec![],
            all_object_valid: false,
            inherited_fields_stale: false,
        };

        assert_eq!(
            dcf.to_string(),
            "\
            import views\n\
            from views import DistributedDonut\n\
            from views import Class, ClassAI, ClassOV\n\
            \n\
            ",
        );
    }
}

/// Contains intermediate DC file structure and logic
/// for semantic analysis as the DC file is being built.
pub(crate) mod interim {
    use super::*;
    use crate::dclass::interim::DClass;
    use crate::parser::error::{Diagnostic, SemanticError};
    use crate::parser::pipeline::PipelineData;
    use anyhow::Result;
    use std::collections::HashSet;

    #[derive(Debug)]
    pub struct PythonImport {
        pub module: String,
        pub symbols: Vec<String>,
    }

    /// DC file structure for internal use by the DC parser.
    #[derive(Debug)]
    pub(crate) struct DCFile {
        pub structs: Vec<DCStruct>,
        pub dclasses: Vec<DClass>,
        pub imports: Vec<PythonImport>,
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
        /// Assigns unique ID to the field for the scope of the entire DC file.
        pub fn add_field(&mut self, _field: DCField) {
            todo!();
        }

        /// Redundancy check for an array of strings that represent view suffixes.
        /// The lexer already generates a specific token type for view suffixes,
        /// and the parser grammar expects this token type, so we already are
        /// guaranteed that the view suffixes are valid.
        fn check_view_suffixes(data: &mut PipelineData, view_suffixes: &ast::ViewSuffixes) {
            let mut recorded_suffixes: HashSet<String> = HashSet::default();

            for view_suffix in view_suffixes {
                if !recorded_suffixes.insert(view_suffix.view.clone()) {
                    let diag: Diagnostic = Diagnostic::error(
                        view_suffix.span,
                        data.current_stage(),
                        data.current_file(),
                        SemanticError::RedundantViewSuffix(view_suffix.view.clone()),
                    );

                    data.emit_diagnostic(diag.into())
                        .expect("Failed to emit diagnostic.");
                }
            }
        }

        /// 'Untangles' a [`ast::PythonImport`], which represents a python import line,
        /// into one or more [`PythonImport`] structures, which represent symbol imports
        /// from a python module (with view suffixes applied) and adds them to the DC file.
        pub fn add_python_import(&mut self, data: &mut PipelineData, import: ast::PythonImport) {
            let mut imports: Vec<PythonImport> = vec![];
            let mut class_symbols: Vec<String> = vec![import.class.symbol.clone()];

            // check view suffixes
            Self::check_view_suffixes(data, &import.module.symbol_views);
            Self::check_view_suffixes(data, &import.class.symbol_views);

            // Separates "Class/AI/OV" to ["Class", "ClassAI", "ClassOV"]
            if !import.class.symbol_views.is_empty() {
                for class_suffix in &import.class.symbol_views {
                    class_symbols.push(import.class.symbol.clone() + &class_suffix.view);
                }
            }

            // Handles e.g. "from module/AI/OV/UD import DistributedThing/AI/OV/UD"
            if !import.module.symbol_views.is_empty() {
                let mut c_symbol: String = class_symbols.first().unwrap().clone();

                imports.push(PythonImport {
                    module: import.module.symbol.clone(),
                    symbols: vec![c_symbol],
                });

                for (i, module_suffix) in import.module.symbol_views.into_iter().enumerate() {
                    let full_import: String = import.module.symbol.clone() + &module_suffix.view;

                    if (class_symbols.len() - 1) <= i {
                        c_symbol = class_symbols.last().unwrap().clone();
                    } else {
                        c_symbol = class_symbols.get(i + 1).unwrap().clone();
                    }

                    imports.push(PythonImport {
                        module: full_import,
                        symbols: vec![c_symbol],
                    });
                }
            } else {
                // No view suffixes for the module symbol, so just push the symbol.
                imports.push(PythonImport {
                    module: import.module.symbol,
                    symbols: class_symbols,
                });
            }

            for imp in imports {
                self.imports.push(imp);
            }
        }

        pub fn add_keyword(&mut self, _keyword: DCKeyword) {
            // TODO!
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
