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

//! The DC parser outputs an [`Abstract Syntax Tree`], which is just a big
//! nested structure that defines the declarations in the DC file. At runtime,
//! the Donet daemon (and its services) need a class hierarchy structure in
//! memory to access while processing network messages.
//!
//! This source file defines the process of taking in the DC file abstract
//! syntax tree as input and generating an output of a class hierarchy structure,
//! where each class has pointers to its children, and vice versa, with methods
//! that make it easy for the Donet daemon to look up information on the DC contract
//! at runtime in order to understand the network messages it receives.
//!
//! [`Abstract Syntax Tree`]: https://en.wikipedia.org/wiki/Abstract_syntax_tree

use super::ast;
use crate::dcfile::*;

/// Takes in the [`Abstract Syntax Tree`] from the DC parser and outputs a
/// [`crate::dcfile::DCFile`] immutable structure with a static lifetime.
///
/// [`Abstract Syntax Tree`]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
pub fn generate_dcf_structure<'a>(_: ast::Root) -> DCFile<'a> {
    let dc_file: DCFile = DCFile::new(vec![], vec![], vec![], vec![], vec![], vec![], true, false);

    /*for type_declaration in ast.type_declarations {
        match type_declaration {
            ast::TypeDeclaration::PythonImport(imports) => {
                //for import in imports {
                    //dc_file.borrow_mut().add_python_import(import);
                //}
            }
            ast::TypeDeclaration::KeywordType(_) => {
                //dc_file.borrow_mut().add_keyword(keyword);
            }
            ast::TypeDeclaration::StructType(_) => {}
            ast::TypeDeclaration::DClassType(_) => {}
            ast::TypeDeclaration::TypedefType(_) => {}
        }
    }*/
    // TODO: maybe properly handle semantic errors in the future
    //assert!(dc_file.borrow().semantic_analysis().is_ok());

    dc_file
}
