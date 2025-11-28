/*
    This file is part of Donet.

    Copyright © 2024-2025 Max Rodriguez <me@maxrdz.com>

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

use crate::dckeyword::DCKeyword;
use crate::dclass::DClass;
use crate::dcstruct::DCStruct;
use crate::dctype::DCTypeDefinition;
use crate::globals;
use crate::hashgen::*;
use crate::parser::ast;

/// Represents a Python-style import statement in the DC file.
#[derive(Debug, Clone)]
pub struct DCPythonImport {
    pub module: String,
    pub symbols: Vec<String>,
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
pub struct DCFile {
    pub(crate) imports: Vec<DCPythonImport>,
    pub(crate) type_defs: Vec<DCTypeDefinition>,
    pub(crate) keywords: Vec<DCKeyword>,
    pub(crate) structs: Vec<DCStruct>,
    pub(crate) dclasses: Vec<DClass>,
    pub(crate) all_object_valid: bool,
    pub(crate) inherited_fields_stale: bool,
}

impl std::fmt::Display for DCFile {
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
        if !self.keywords.is_empty() {
            writeln!(f)?;
        }
        // Print Keyword definitions
        for kw in &self.keywords {
            kw.fmt(f)?;
            writeln!(f)?;
        }
        if !self.structs.is_empty() {
            writeln!(f)?;
        }
        // Print Structs
        for strukt in &self.structs {
            strukt.fmt(f)?;
            writeln!(f)?;
        }
        if !self.dclasses.is_empty() {
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

impl LegacyDCHash for DCFile {
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        hashgen.add_int(self.get_num_dclasses().try_into().unwrap());

        for dclass in &self.dclasses {
            dclass.generate_hash(hashgen);
        }

        hashgen.add_int(self.get_num_structs().try_into().unwrap());

        for strukt in &self.structs {
            strukt.generate_hash(hashgen);
        }

        hashgen.add_int(self.get_num_keywords().try_into().unwrap());

        for kw in &self.keywords {
            kw.generate_hash(hashgen);
        }
    }
}

impl DCFile {
    /// Returns a 32-bit hash index associated with this file.  This number is
    /// guaranteed to be consistent if the contents of the file have not changed,
    /// and it is very likely to be different if the contents of the file do change.
    pub fn get_legacy_hash(&self) -> globals::DCFileHash {
        let mut hashgen: DCHashGenerator = DCHashGenerator::default();

        self.generate_hash(&mut hashgen);
        hashgen.get_hash()
    }

    /// Returns a string with the hash as a pretty format hexadecimal.
    pub fn get_pretty_hash(&self) -> String {
        format!("0x{:0width$x}", self.get_legacy_hash(), width = 8) // 2 hex / byte = 8 hex
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
        self.keywords.len()
    }

    pub fn get_keyword(&self, _index: usize) -> &DCKeyword {
        todo!();
    }

    pub fn has_keyword(&self, _keyword: String) -> bool {
        todo!();
    }

    // ---------- Distributed Class ---------- //

    pub fn get_num_dclasses(&self) -> usize {
        self.dclasses.len()
    }

    pub fn get_dclass(&self, _index: usize) -> &DClass {
        todo!();
    }

    pub fn get_dclass_by_id(&self, id: globals::DClassId) -> &DClass {
        self.dclasses.get(usize::from(id)).unwrap()
    }

    pub fn get_dclass_by_name(&self, _name: &str) -> &DClass {
        todo!();
    }

    // ---------- DC Struct ---------- //

    pub fn get_num_structs(&self) -> usize {
        self.structs.len()
    }

    pub fn get_struct(&self, _index: usize) -> &DCStruct {
        todo!();
    }
}

#[cfg(test)]
mod tests {
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

        let dcf: DCFile = DCFile {
            structs: vec![],
            dclasses: vec![],
            imports,
            keywords: vec![],
            type_defs: vec![],
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

pub(crate) mod semantics {
    use super::ast;
    use crate::parser::error::{Diagnostic, SemanticError};
    use crate::parser::pipeline::{PipelineData, TopLevelSymbol};
    use std::collections::HashSet;

    /// Redundancy check for an array of strings that represent view suffixes.
    /// The lexer already generates a specific token type for view suffixes,
    /// and the parser grammar expects this token type, so we already are
    /// guaranteed that the view suffixes are valid.
    fn analyze_view_suffixes(pipeline: &mut PipelineData, view_suffixes: &ast::ViewSuffixes) {
        let mut recorded_suffixes: HashSet<String> = HashSet::default();

        for view_suffix in view_suffixes {
            if !recorded_suffixes.insert(view_suffix.view.clone()) {
                let diag: Diagnostic = Diagnostic::error(
                    view_suffix.span,
                    pipeline,
                    SemanticError::RedundantViewSuffix(view_suffix.view.clone()),
                );

                pipeline
                    .emit_diagnostic(diag.into())
                    .expect("Failed to emit diagnostic.");
            }
        }
    }

    pub fn analyze_python_import(pipeline: &mut PipelineData, import: &ast::PythonImport) {
        // check view suffixes
        analyze_view_suffixes(pipeline, &import.module.symbol_views);
        analyze_view_suffixes(pipeline, &import.class.symbol_views);
    }

    pub fn analyze_keyword(pipeline: &mut PipelineData, keyword: &ast::KeywordDefinition) {
        let already_defined: bool = pipeline
            .dc_data
            .symbol_exists(&keyword.identifier, TopLevelSymbol::KeywordDef);

        if already_defined {
            let diag: Diagnostic = Diagnostic::error(
                keyword.span,
                pipeline,
                SemanticError::AlreadyDefined(keyword.identifier.clone()),
            );

            pipeline
                .emit_diagnostic(diag.into())
                .expect("Failed to emit diagnostic.");
            return;
        }
        // add this keyword definition to our symbol map
        pipeline
            .dc_data
            .register_symbol(keyword.identifier.clone(), TopLevelSymbol::KeywordDef);
    }

    pub fn analyze_typedef(pipeline: &mut PipelineData, typedef: &ast::TypeDefinition) {
        //let new_td: DCTypeDefinition = typedef.into();
        // TODO: semantic checks (e.g. typedef Struct Name; -> verify Struct exists)
        //self.typedefs.push(new_td);
    }

    pub fn analyze_dclass(pipeline: &mut PipelineData, dclass: &ast::DClass) {
        //self.dclasses.push(dclass);
    }

    pub fn analyze_struct(pipeline: &mut PipelineData, strukt: &ast::Struct) {
        //let new_struct: DCStruct = strukt.into();
        //self.structs.push(new_struct);
    }
}

pub(crate) mod generation {
    use super::DCPythonImport;
    use crate::parser::ast;

    /// 'Untangles' a [`ast::PythonImport`], which represents a python import line,
    /// into one or more [`DCPythonImport`] structures, which represent symbol imports
    /// from a python module (with view suffixes applied) and adds them to the DC tree.
    ///
    pub fn add_python_import(import: &ast::PythonImport) -> Vec<DCPythonImport> {
        let mut imports: Vec<DCPythonImport> = vec![];
        let mut class_symbols: Vec<String> = vec![import.class.symbol.clone()];

        // Separates "Class/AI/OV" to ["Class", "ClassAI", "ClassOV"]
        if !import.class.symbol_views.is_empty() {
            for class_suffix in &import.class.symbol_views {
                class_symbols.push(import.class.symbol.clone() + &class_suffix.view);
            }
        }

        // Handles e.g. "from module/AI/OV/UD import DistributedThing/AI/OV/UD"
        if !import.module.symbol_views.is_empty() {
            let mut c_symbol: String = class_symbols.first().unwrap().clone();

            imports.push(DCPythonImport {
                module: import.module.symbol.clone(),
                symbols: vec![c_symbol],
            });

            for (i, module_suffix) in import.module.symbol_views.iter().enumerate() {
                let full_import: String = import.module.symbol.clone() + &module_suffix.view;

                if (class_symbols.len() - 1) <= i {
                    c_symbol = class_symbols.last().unwrap().clone();
                } else {
                    c_symbol = class_symbols.get(i + 1).unwrap().clone();
                }

                imports.push(DCPythonImport {
                    module: full_import,
                    symbols: vec![c_symbol],
                });
            }
        } else {
            // No view suffixes for the module symbol, so just push the symbol.
            imports.push(DCPythonImport {
                module: import.module.symbol.clone(),
                symbols: class_symbols,
            });
        }

        imports
    }
}
