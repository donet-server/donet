/*
    This file is part of Donet.

    Copyright © 2024 Max Rodriguez <me@maxrdz.com>

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
use crate::dconfig::*;
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

/// A different enumerator representing DC Field types used
/// for DC Structs, since they cannot contain DC Atomic Fields.
#[derive(Debug)]
pub enum StructField<'dc> {
    Field(DCField<'dc>),
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
    field_type: Option<DCTypeDefinition>,
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

impl DCFileConfigAccessor for DCField<'_> {
    fn get_dc_config(&self) -> &DCFileConfig {
        match self.parent_element {
            FieldParent::DClass(dc) => dc.get_dc_config(),
            FieldParent::Strukt(s) => s.get_dc_config(),
        }
    }
}

impl LegacyDCHash for DCField<'_> {
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        self.keyword_list.generate_hash(hashgen);
        self.field_type.clone().unwrap().generate_hash(hashgen);

        // It shouldn't be necessary to explicitly add the field ID
        // to the hash--this is computed based on the relative
        // position of this field with the other fields, so adding it
        // explicitly will be redundant. However, the field name is
        // significant.
        hashgen.add_string(self.field_name.clone());

        // The field ID is added to the hash here, since we need to
        // ensure the hash code comes out different in the
        // DC_MULTIPLE_INHERITANCE case.
        if self.get_dc_config().dc_multiple_inheritance {
            hashgen.add_int(i32::from(self.field_id));
        }
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
    pub fn set_field_id(&mut self, id: globals::FieldId) {
        self.field_id = id
    }

    #[inline(always)]
    pub fn set_field_name(&mut self, name: String) {
        self.field_name = name
    }

    pub fn set_field_type(&mut self, dtype: DCTypeDefinition) {
        self.field_type = Some(dtype);
        self.has_default_value = false;
        self.default_value = vec![];
    }

    pub fn set_field_keyword_list(&mut self, kw_list: DCKeywordList<'dc>) {
        self.keyword_list = kw_list;
    }

    pub fn set_default_value(&mut self, value: Vec<u8>) {
        self.default_value = value;
        self.has_default_value = true;
        self.default_value_stale = false;
    }

    #[inline(always)]
    pub fn set_bogus_field(&mut self, is_bogus: bool) {
        self.bogus_field = is_bogus
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
