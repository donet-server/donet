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
use crate::hashgen::DCHashGenerator;

/// A field of a Distributed Class. The DCField struct is a base for
/// struct and dclass fields. In the DC language, there are three types
/// of field declarations, which are: plain fields, atomic, and molecular.
#[derive(Debug)]
pub struct DCField<'dc> {
    keyword_list: DCKeywordList,
    dclass: Option<&'dc DClass<'dc>>,
    strukt: Option<&'dc DCStruct>, // 'strukt' due to reserved keyword
    field_name: String,
    field_id: globals::FieldId,
    field_type: DCTypeDefinition,
    parent_is_dclass: bool,
    default_value_stale: bool,
    has_default_value: bool,
    default_value: Vec<u8>, // stored as byte array
    bogus_field: bool,
}

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

/// Macro for Panda historical keywords inline functions.
macro_rules! has_keyword {
    ($self:ident, $i:literal) => {
        $self
            .keyword_list
            .has_keyword(IdentifyKeyword::ByName($i.to_owned()))
    };
}

impl<'dc> DCField<'dc> {
    pub(crate) fn new(name: &str, dtype: DCTypeDefinition) -> Self {
        Self {
            keyword_list: DCKeywordList::new(),
            dclass: None,
            strukt: None,
            field_name: name.to_owned(),
            field_id: 0_u16,
            field_type: dtype,
            parent_is_dclass: false,
            default_value_stale: false,
            has_default_value: false,
            default_value: vec![],
            bogus_field: false,
        }
    }

    /// Accumulates the properties of this DC element into the file hash.
    pub fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        self.keyword_list.generate_hash(hashgen);
        self.field_type.generate_hash(hashgen);

        // It shouldn't be necessary to explicitly add the field ID
        // to the hash--this is computed based on the relative
        // position of this field with the other fields, so
        // adding it explicitly will be redundant.  However,
        // the field name is significant.
        hashgen.add_string(self.field_name.clone());

        // The field ID is added to the hash here, since we need to ensure
        // the hash code comes out different in the DC_MULTIPLE_INHERITANCE case.
        if globals::DC_MULTIPLE_INHERITANCE {
            hashgen.add_int(i32::from(self.field_id));
        }
    }

    #[inline(always)]
    pub fn get_field_id(&self) -> globals::FieldId {
        self.field_id
    }

    #[inline(always)]
    pub fn get_field_name(&self) -> String {
        self.field_name.clone()
    }

    pub fn get_dclass(&self) -> &'static DClass {
        assert!(self.parent_is_dclass);
        self.dclass.unwrap()
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
        self.field_type = dtype;
        self.has_default_value = false;
        self.default_value = vec![];
    }

    pub fn set_field_keyword_list(&mut self, kw_list: DCKeywordList) {
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

    pub fn set_parent_struct(&mut self, parent: &'dc DCStruct) {
        assert!(self.dclass.is_none());
        self.strukt = Some(parent);
    }

    pub fn set_parent_dclass(&mut self, parent: &'dc DClass<'dc>) {
        assert!(self.strukt.is_none());
        self.dclass = Some(parent);
    }

    #[inline(always)]
    pub fn has_default_value(&self) -> bool {
        self.has_default_value
    }

    pub fn validate_ranges(&self, _packed_data: &Datagram) -> bool {
        todo!()
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

    fn refresh_default_value(&self) {
        todo!()
    }
}
