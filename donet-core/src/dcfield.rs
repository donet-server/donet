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

//! Base data model for DC Field elements. Alone, it represents
//! an attribute of a structure or Distributed Class.

use crate::datagram::datagram::Datagram;
use crate::dcatomic::DCAtomicField;
use crate::dckeyword::{DCKeywordList, IdentifyKeyword};
use crate::dclass::DClass;
use crate::dcmolecular::DCMolecularField;
use crate::dcstruct::DCStruct;
use crate::dctype::DCTypeDefinition;
use crate::globals;
use crate::hashgen::*;

/// Enumerator representing the 3 types of fields that inherit DC Field,
/// which can legally be declared within a Distributed Class.
///
/// Plain DC Fields represent a property, or member, of a structure
/// or class. DC fields have a data type assigned to them.
///
/// DC Atomic Fields represent a method of a Distributed Class, which
/// is always implemented as a remote procedure call (RPC). Unlike
/// attribute fields, atomic fields cannot be declared within structs.
///
/// DC Molecular Fields represent a collection of one or more
/// DC Atomic Fields as one field under one identifier. The parameters
/// of a molecular field are the parameters of all the fields it
/// represents, joined together in the order in which they were declared
/// when the molecular field was declared.
#[derive(Debug)]
pub enum ClassField<'dc> {
    Field(DCField<'dc>),
    Atomic(DCAtomicField<'dc>),
    Molecular(DCMolecularField<'dc>),
}

/// A DC field element can be declared within a dclass or a
/// struct declaration. The DC field element must have a
/// reference to its parent, which is stored in this enum type.
#[derive(Debug)]
pub enum FieldParent<'dc> {
    DClass(&'dc DClass<'dc>),
    Strukt(&'dc DCStruct<'dc>), // 'strukt' due to reserved keyword
}

/// Macro for Panda historical keywords inline functions.
macro_rules! has_keyword {
    ($self:ident, $i:literal) => {
        $self
            .keyword_list
            .has_keyword(IdentifyKeyword::ByName($i.to_owned()))
    };
}

/// A field of a Distributed Class. The DCField struct is a base for
/// struct and dclass fields. In the DC language, there are three types
/// of field declarations, which are: plain fields, atomic, and molecular.
#[derive(Debug)]
pub struct DCField<'dc> {
    keyword_list: DCKeywordList<'dc>,
    parent_element: FieldParent<'dc>,
    field_name: String,
    field_id: globals::FieldId,
    field_type: DCTypeDefinition,
    default_value_stale: bool,
    has_default_value: bool,
    default_value: Vec<u8>, // stored as byte array
    bogus_field: bool,
}

impl std::fmt::Display for DCField<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "TODO")
    }
}

impl LegacyDCHash for DCField<'_> {
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        hashgen.add_int(self.field_id.into());
        hashgen.add_string(self.field_name.clone());

        self.field_type.clone().generate_hash(hashgen);
        self.keyword_list.generate_hash(hashgen);
    }
}

impl<'dc> DCField<'dc> {
    #[inline(always)]
    pub fn get_field_id(&self) -> globals::FieldId {
        self.field_id
    }

    #[inline(always)]
    pub fn get_field_name(&self) -> String {
        self.field_name.clone()
    }

    /// Gets the parent DClass element reference.
    ///
    /// Panics if this field's parent element is not a DClass.
    pub fn get_dclass(&self) -> &'dc DClass {
        match self.parent_element {
            FieldParent::DClass(dclass_ref) => dclass_ref,
            FieldParent::Strukt(_) => panic!("Field parent is not a DClass."),
        }
    }

    #[inline(always)]
    pub fn has_default_value(&self) -> bool {
        self.has_default_value
    }

    pub fn validate_ranges(&self, _packed_data: &Datagram) -> bool {
        todo!()
    }

    /// Given a blob that represents the packed data for this field, returns a
    /// string formatting it for human consumption.
    pub fn format_packed_data(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _data: &[u8],
        _show_field_names: bool,
    ) -> std::fmt::Result {
        f.write_str("TODO") // TODO
    }

    #[inline(always)]
    pub fn is_bogus_field(&self) -> bool {
        self.bogus_field
    }

    #[inline(always)]
    pub fn is_required(&self) -> bool {
        has_keyword!(self, "required")
    }

    #[inline(always)]
    pub fn is_broadcast(&self) -> bool {
        has_keyword!(self, "broadcast")
    }

    #[inline(always)]
    pub fn is_ram(&self) -> bool {
        has_keyword!(self, "ram")
    }

    #[inline(always)]
    pub fn is_db(&self) -> bool {
        has_keyword!(self, "db")
    }

    #[inline(always)]
    pub fn is_clsend(&self) -> bool {
        has_keyword!(self, "clsend")
    }

    #[inline(always)]
    pub fn is_clrecv(&self) -> bool {
        has_keyword!(self, "clrecv")
    }

    #[inline(always)]
    pub fn is_ownsend(&self) -> bool {
        has_keyword!(self, "ownsend")
    }

    #[inline(always)]
    pub fn is_ownrecv(&self) -> bool {
        has_keyword!(self, "ownrecv")
    }

    #[inline(always)]
    pub fn is_airecv(&self) -> bool {
        has_keyword!(self, "airecv")
    }

    fn _refresh_default_value(&self) {
        todo!()
    }
}

pub(crate) mod interim {
    use crate::parser::ast;

    #[derive(Debug)]
    pub struct DCField {}

    impl From<ast::StructField> for DCField {
        fn from(value: ast::StructField) -> Self {
            Self {}
        }
    }
}
