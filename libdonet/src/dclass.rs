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

//! Data model for Distributed Class definitions in the DC file.
//! Stores DC Fields and tracks class hierarchy.

use crate::dcatomic::DCAtomicField;
use crate::dcfield::ClassField;
use crate::dcfile::DCFile;
use crate::dconfig::*;
use crate::globals;
use crate::hashgen::*;
use multimap::MultiMap;

pub type FieldName2Field<'dc> = MultiMap<String, &'dc ClassField<'dc>>;
pub type FieldId2Field<'dc> = MultiMap<globals::FieldId, &'dc ClassField<'dc>>;

/// Represents a Distributed Class defined in the DC file.
/// Contains a map of DC Fields, as well as atomic and
/// molecular fields that are declared within the class.
/// Also stores other properties such as its hierarchy.
#[derive(Debug)]
pub struct DClass<'dc> {
    dcfile: &'dc DCFile<'dc>,
    class_name: String,
    class_id: globals::DClassId,
    is_bogus_class: bool,
    class_parents: Vec<&'dc DClass<'dc>>,
    constructor: Option<&'dc DCAtomicField<'dc>>,
    fields: Vec<&'dc ClassField<'dc>>,
    inherited_fields: Vec<&'dc ClassField<'dc>>,
    field_name_2_field: FieldName2Field<'dc>,
    field_id_2_field: FieldId2Field<'dc>,
}

impl<'dc> std::fmt::Display for DClass<'dc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dclass ")?;
        f.write_str(&self.get_name())?;

        if !self.class_parents.is_empty() {
            write!(f, " : ")?;

            for (i, parent) in self.class_parents.iter().enumerate() {
                parent.fmt(f)?;

                if i != self.class_parents.len() - 1 {
                    write!(f, ", ")?;
                }
            }
        }
        write!(f, " {{  // index ")?;
        self.class_id.fmt(f)?;
        writeln!(f)?;

        if let Some(constructor) = self.constructor {
            constructor.fmt(f)?;
        }

        for field in &self.fields {
            match field {
                ClassField::Atomic(cf) => cf.fmt(f)?,
                ClassField::Field(cf) => cf.fmt(f)?,
                ClassField::Molecular(cf) => cf.fmt(f)?,
            }
        }
        writeln!(f, "}};")
    }
}

impl<'dc> DCFileConfigAccessor for DClass<'dc> {
    fn get_dc_config(&self) -> &DCFileConfig {
        self.dcfile.get_dc_config()
    }
}

impl<'dc> LegacyDCHash for DClass<'dc> {
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        hashgen.add_string(self.get_name());
        hashgen.add_int(self.get_num_parents().try_into().unwrap());

        for parent in &self.class_parents {
            {
                hashgen.add_int(i32::from(parent.get_dclass_id()));
            }

            if let Some(constructor) = &self.constructor {
                constructor.generate_hash(hashgen);
            }
        }
        hashgen.add_int(self.fields.len().try_into().unwrap());

        for field in &self.fields {
            match field {
                ClassField::Field(field) => field.generate_hash(hashgen),
                ClassField::Atomic(atomic) => atomic.generate_hash(hashgen),
                ClassField::Molecular(molecular) => molecular.generate_hash(hashgen),
            }
        }
    }
}

impl<'dc> DClass<'dc> {
    pub fn get_field_by_name(&self, name: &str) -> Option<&'dc ClassField> {
        match self.field_name_2_field.get(name) {
            Some(pointer) => Some(pointer),
            None => None,
        }
    }

    #[inline(always)]
    pub fn get_name(&self) -> String {
        self.class_name.clone()
    }

    #[inline(always)]
    pub fn get_dclass_id(&self) -> globals::DClassId {
        self.class_id
    }

    #[inline(always)]
    pub fn get_num_parents(&self) -> usize {
        self.class_parents.len()
    }

    #[inline(always)]
    pub fn get_parent(&self, index: usize) -> Option<&'static DClass> {
        // copy the reference inside the option instead of a reference to the reference
        self.class_parents.get(index).cloned()
    }

    #[inline(always)]
    pub fn has_constructor(&self) -> bool {
        self.constructor.is_some()
    }

    #[inline(always)]
    pub fn get_constructor(&self) -> Option<&'dc DCAtomicField> {
        if let Some(atomic) = &self.constructor {
            Some(atomic)
        } else {
            None
        }
    }
}

/// Contains intermediate DClass structure and logic
/// for semantic analysis as the DClass is being built.
pub(crate) mod interim {
    use crate::globals;
    use crate::parser::ast;
    use crate::parser::lexer::Span;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    pub struct DClass {
        pub span: Span,
        pub identifier: String,
        pub parents: Vec<String>,
        pub fields: ast::ClassFields,
        pub class_id: globals::DClassId,
        pub is_bogus_class: bool,
        pub class_parents: Vec<Rc<RefCell<DClass>>>,
    }

    impl DClass {
        #[inline(always)]
        pub fn add_parent(&mut self, parent: Rc<RefCell<DClass>>) {
            self.class_parents.push(parent);
        }

        /// Adds a newly allocated DC field to this class. The field structure
        /// in memory is moved into ownership of this class structure, and is
        /// wrapped in a [`std::cell::RefCell`] and an [`std::rc::Rc`] pointer
        ///to pass references to other elements, such as molecular fields.
        pub fn add_class_field(&mut self, field: ast::AtomicOrMolecular) {
            self.is_bogus_class = false;
            self.fields.push(field);
        }
    }
}
