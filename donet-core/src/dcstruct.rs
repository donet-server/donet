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

//! Data model representing a DC Struct element.
//! Very similar to a DClass element, but does not allow
//! declaring atomic or molecular fields.

use crate::dcfield::DCField;
use crate::dcfile::DCFile;
use crate::globals;
use crate::hashgen::*;
use crate::parser::ast;
use multimap::MultiMap;

pub type FieldName2Field<'dc> = MultiMap<String, &'dc DCField<'dc>>;
pub type FieldId2Field<'dc> = MultiMap<globals::FieldId, &'dc DCField<'dc>>;

#[derive(Debug, Clone)]
pub struct DCStruct<'dc> {
    pub dcfile: &'dc DCFile<'dc>,
    pub identifier: String,
    pub id: globals::DClassId,
    pub fields: Vec<&'dc DCField<'dc>>,
    pub field_name_2_field: FieldName2Field<'dc>,
    pub field_id_2_field: FieldId2Field<'dc>,
    /// Whether this struct has any fields that have a constraint. (e.g. ranges)
    pub has_range: bool,
}

impl std::fmt::Display for DCStruct<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "struct ")?;
        f.write_str(&self.identifier)?;

        write!(f, " {{  // index ")?;
        self.id.fmt(f)?;
        writeln!(f)?;

        for field in &self.fields {
            field.fmt(f)?;
        }
        writeln!(f, "}};")
    }
}

impl LegacyDCHash for DCStruct<'_> {
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        hashgen.add_string(self.identifier.clone());
        hashgen.add_int(self.fields.len().try_into().unwrap());

        for field in &self.fields {
            field.generate_hash(hashgen);
        }
    }
}

/// Contains intermediate DC struct element structure and logic
/// for semantic analysis as the DC struct is being built.
pub(crate) mod interim {
    use super::ast;
    use crate::dcfield::interim::DCField;
    use crate::parser::lexer::Span;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    pub struct DCStruct {
        pub span: Span,
        pub identifier: String,
        pub fields: Vec<Rc<RefCell<DCField>>>,
    }

    impl From<ast::Struct> for DCStruct {
        fn from(value: ast::Struct) -> Self {
            let mut fields: Vec<Rc<RefCell<DCField>>> = vec![];

            for f in value.fields {
                fields.push(Rc::new(RefCell::new(f.into())));
            }
            Self {
                span: value.span,
                identifier: value.identifier,
                fields: fields,
            }
        }
    }
}
