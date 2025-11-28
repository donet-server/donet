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

//! Data model for Distributed Class definitions in the DC file.
//! Stores DC Fields and tracks class hierarchy.

use crate::dcatomic::DCAtomicField;
use crate::dcfield::ClassField;
use crate::globals;
use crate::hashgen::*;

/// Represents a Distributed Class defined in the DC file.
/// Contains a map of DC Fields, as well as atomic and
/// molecular fields that are declared within the class.
/// Also stores other properties such as its hierarchy.
#[derive(Debug)]
pub struct DClass {
    class_name: String,
    class_id: globals::DClassId,
    is_bogus_class: bool,
    constructor: Option<DCAtomicField>,
    fields: Vec<ClassField>,
}

impl std::fmt::Display for DClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dclass ")?;
        f.write_str(&self.get_name())?;

        /*if !self.class_parents.is_empty() {
            write!(f, " : ")?;

            for (i, parent) in self.class_parents.iter().enumerate() {
                parent.fmt(f)?;

                if i != self.class_parents.len() - 1 {
                    write!(f, ", ")?;
                }
            }
        }*/
        write!(f, " {{  // index ")?;
        self.class_id.fmt(f)?;
        writeln!(f)?;

        if let Some(constructor) = &self.constructor {
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

impl LegacyDCHash for DClass {
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        hashgen.add_string(self.get_name());
        //hashgen.add_int(self.get_num_parents().try_into().unwrap());

        // Hash our inheritance tree TODO!
        //for parent in &self.class_parents {
        //    hashgen.add_int(i32::from(parent.get_dclass_id()));
        //}

        if let Some(constructor) = &self.constructor {
            constructor.generate_hash(hashgen);
        }
        hashgen.add_int(self.fields.len().try_into().unwrap());

        // Hash our DC fields
        for field in &self.fields {
            match field {
                ClassField::Field(field) => field.generate_hash(hashgen),
                ClassField::Atomic(atomic) => atomic.generate_hash(hashgen),
                ClassField::Molecular(molecular) => molecular.generate_hash(hashgen),
            }
        }
    }
}

impl DClass {
    #[inline(always)]
    pub fn get_name(&self) -> String {
        self.class_name.clone()
    }

    #[inline(always)]
    pub fn get_dclass_id(&self) -> globals::DClassId {
        self.class_id
    }

    #[inline(always)]
    pub fn has_constructor(&self) -> bool {
        self.constructor.is_some()
    }

    #[inline(always)]
    pub fn get_constructor(&self) -> Option<&DCAtomicField> {
        if let Some(atomic) = &self.constructor {
            Some(atomic)
        } else {
            None
        }
    }
}
