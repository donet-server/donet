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
use std::cell::RefCell;
use std::rc::Rc;

/// Takes in the [`Abstract Syntax Tree`] from the DC parser and outputs a
/// [`crate::dcfile::DCFile`] structure wrapped in a [`std::rc::Rc`] pointer.
///
/// [`Abstract Syntax Tree`]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
pub fn generate_dcf_structure(ast: ast::Root) -> Rc<RefCell<DCFile>> {
    let dc_file: Rc<RefCell<DCFile>> = Rc::new(RefCell::new(DCFile::new()));

    for type_declaration in ast.type_declarations {
        match type_declaration {
            ast::TypeDeclaration::PythonImport(imports) => {
                for import in imports {
                    dc_file.borrow_mut().add_python_import(import);
                }
            }
            ast::TypeDeclaration::KeywordType(keyword) => {
                dc_file.borrow_mut().add_keyword(keyword);
            }
            ast::TypeDeclaration::StructType(_) => {}
            ast::TypeDeclaration::SwitchType(_) => {}
            ast::TypeDeclaration::DClassType(mut dclass) => {
                dclass.set_dcfile(Rc::clone(&dc_file));

                let next_class_id: usize = dc_file.borrow_mut().get_num_dclasses();
                dclass.set_dclass_id(next_class_id.try_into().unwrap());

                dc_file.borrow_mut().add_dclass(dclass);
            }
            ast::TypeDeclaration::TypedefType(_) => {}
        }
    }
    // TODO: maybe properly handle semantic errors in the future
    assert!(dc_file.borrow().semantic_analysis().is_ok());

    dc_file
}
