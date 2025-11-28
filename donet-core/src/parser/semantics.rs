/*
    This file is part of Donet.

    Copyright © 2024-2025 Max Rodriguez

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

//! The DC parser outputs an [`Abstract Syntax Tree`], which is just a big
//! nested structure that defines the declarations in the DC file. At runtime,
//! the Donet daemon (and its services) need a class hierarchy structure in
//! memory to access while processing network messages.
//!
//! This source file defines the process of taking in the DC file abstract
//! syntax tree as input and generating an output of a class hierarchy structure,
//! where each class has methods that make it easy for the Donet daemon to look up
//! information on the DC contract at runtime in order to understand the
//! network messages it receives.
//!
//! [`Abstract Syntax Tree`]: https://en.wikipedia.org/wiki/Abstract_syntax_tree

use super::ast;
use super::error::DCReadError;
use super::PipelineData;
use crate::dcfile;
use anyhow::Result;

/// Takes in the [`Abstract Syntax Trees`] from the last stage of the pipeline
/// and outputs error diagnostics if any issues are found.
///
/// [`Abstract Syntax Trees`]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
///
pub fn semantic_analyzer(pipeline: &mut PipelineData) -> Result<(), DCReadError> {
    // tell the pipeline we are moving onto the next stage
    pipeline.next_stage();

    // Iterate through all ASTs and analyze it.
    for ast in pipeline.syntax_trees.clone() {
        for type_declaration in ast.type_declarations.iter() {
            match type_declaration {
                ast::TypeDeclaration::PythonImport(import) => {
                    dcfile::semantics::analyze_python_import(pipeline, import);
                }
                ast::TypeDeclaration::KeywordType(keyword) => {
                    dcfile::semantics::analyze_keyword(pipeline, keyword);
                }
                ast::TypeDeclaration::StructType(_strukt) => {
                    // TODO
                }
                ast::TypeDeclaration::DClassType(_dclass) => {
                    // TODO
                }
                ast::TypeDeclaration::TypedefType(_typedef) => {
                    // TODO
                }
            }
        }
        pipeline.next_file(); // tell the pipeline we are processing the next file
    }

    if pipeline.failing() {
        Err(DCReadError::Semantic)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_dc;
    use dcfile::DCPythonImport;

    #[test]
    fn python_imports() {
        let dc_string: &str = "
            from views import *
            from views import DistributedDonut
            from views import Class/AI/OV
        ";

        let dcf: dcfile::DCFile = read_dc(dc_string.into()).expect("Failed to parse syntax.");

        let num_imports: usize = dcf.get_num_imports();
        assert_eq!(num_imports, 3);

        let symbols: Vec<Vec<String>> = vec![
            vec!["*".into()],
            vec!["DistributedDonut".into()],
            vec!["Class".into(), "ClassAI".into(), "ClassOV".into()],
        ];

        for index in 0..num_imports - 1 {
            let import: &DCPythonImport = dcf.get_python_import(index);

            assert_eq!(import.module, "views");

            let target_symbols: &Vec<String> = symbols.get(index).unwrap();

            assert_eq!(*target_symbols, import.symbols);
        }
    }

    #[test]
    #[should_panic]
    fn redundant_view_suffix() {
        let dc_string: &str = "
            from views import Class/AI/OV/OV
        ";

        let _ = read_dc(dc_string.into()).expect("Should fail.");
    }

    #[test]
    #[should_panic]
    fn keyword_already_defined() {
        let dc_string: &str = "
            keyword abcdef;
            keyword abcdef;
        ";

        let _ = read_dc(dc_string.into()).expect("Should fail.");
    }
}
