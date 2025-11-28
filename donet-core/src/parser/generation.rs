/*
    This file is part of Donet.

    Copyright © 2025 Max Rodriguez

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

use super::ast;
use super::error::DCReadError;
use super::pipeline::PipelineData;
use crate::dcfile;
use anyhow::Result;

/// Last step of the DC parser pipeline. Generates the final immutable DC element tree
/// data structure for usage by Donet clients and Donet server services.
///
pub fn dc_tree_generation(pipeline: &mut PipelineData) -> dcfile::DCFile {
    // tell the pipeline we are moving onto the next stage
    pipeline.next_stage();

    let mut tree: dcfile::DCFile = dcfile::DCFile {
        imports: vec![],
        type_defs: vec![],
        keywords: vec![],
        structs: vec![],
        dclasses: vec![],
        all_object_valid: true,
        inherited_fields_stale: false,
    };

    // Iterate through all ASTs and add them to our final DC element tree.
    for ast in pipeline.syntax_trees.clone() {
        for type_declaration in ast.type_declarations.iter() {
            match type_declaration {
                ast::TypeDeclaration::PythonImport(import) => {
                    tree.imports.extend(dcfile::generation::add_python_import(import));
                }
                ast::TypeDeclaration::KeywordType(_keyword) => {
                    // TODO
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
    tree
}
