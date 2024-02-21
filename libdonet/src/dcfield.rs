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

use crate::datagram::Datagram;
use crate::dcatomic::DCAtomicField;
use crate::dckeyword::{DCKeywordList, DCKeywordListInterface, IdentifyKeyword};
use crate::dclass::DClass;
use crate::dcmolecular::DCMolecularField;
use crate::dcstruct::DCStruct;
use crate::dctype::{DCTypeDefinition, DCTypeDefinitionInterface};
use crate::globals;
use crate::hashgen::DCHashGenerator;
use std::sync::{Arc, Mutex};

/// A field of a Distributed Class. The DCField struct is a base for
/// struct and dclass fields. In the DC language, there are three types
/// of field declarations, which are: plain fields, atomic, and molecular.
#[derive(Debug)]
pub struct DCField {
    keyword_list: DCKeywordList,
    dclass: Option<Arc<Mutex<DClass>>>,
    _struct: Option<Arc<Mutex<DCStruct>>>, // needs '_' due to reserved keyword
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
/// DC Molecular Fields represent a collection of DC Attribute or
/// DC Atomic Fields as one field under one identifier. The parameters
/// of a molecular field are the parameters of all the fields it
/// represents, joined together in the order in which they were declared
/// when the molecular field was declared.
#[derive(Debug)]
pub enum ClassField {
    Field(DCField),
    Atomic(DCAtomicField),
    Molecular(DCMolecularField),
}

/// A different enumerator representing DC Field types used
/// for DC Structs, since they cannot contain DC Atomic Fields.
#[derive(Debug)]
pub enum StructField {
    Field(DCField),
    Molecular(DCMolecularField),
}

pub trait DCFieldInterface {
    fn new(name: &str, dtype: DCTypeDefinition) -> Self;
    fn generate_hash(&self, hashgen: &mut DCHashGenerator);

    fn get_field_id(&self) -> globals::FieldId;
    fn get_dclass(&self) -> Arc<Mutex<DClass>>;

    fn set_field_id(&mut self, id: globals::FieldId);
    fn set_field_name(&mut self, name: String);
    fn set_field_type(&mut self, dtype: DCTypeDefinition);
    fn set_default_value(&mut self, value: Vec<u8>);
    fn set_bogus_field(&mut self, is_bogus: bool);

    fn set_parent_struct(&mut self, parent: Arc<Mutex<DCStruct>>);
    fn set_parent_dclass(&mut self, parent: Arc<Mutex<DClass>>);

    fn has_default_value(&self) -> bool;
    fn validate_ranges(&self, packed_data: &Datagram) -> bool;

    // Inline functions for Panda historical keywords
    fn is_bogus_field(&self) -> bool;
    fn is_required(&self) -> bool;
    fn is_broadcast(&self) -> bool;
    fn is_ram(&self) -> bool;
    fn is_db(&self) -> bool;
    fn is_clsend(&self) -> bool;
    fn is_clrecv(&self) -> bool;
    fn is_ownsend(&self) -> bool;
    fn is_ownrecv(&self) -> bool;
    fn is_airecv(&self) -> bool;
}

impl DCField {
    fn refresh_default_value(&self) {
        todo!()
    }
}

/// Macro for Panda historical keywords inline functions.
macro_rules! has_keyword {
    ($self:ident, $i:literal) => {
        $self
            .keyword_list
            .has_keyword(IdentifyKeyword::ByName($i.to_owned()))
    };
}

impl DCFieldInterface for DCField {
    fn new(name: &str, dtype: DCTypeDefinition) -> Self {
        Self {
            keyword_list: DCKeywordList::new(),
            dclass: None,
            _struct: None,
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

    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
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
    fn get_field_id(&self) -> globals::FieldId {
        self.field_id
    }

    fn get_dclass(&self) -> Arc<Mutex<DClass>> {
        assert!(self.parent_is_dclass);
        // clone option to unwrap w/o move, and clone Arc to return
        self.dclass.clone().unwrap().clone()
    }

    fn set_field_id(&mut self, id: globals::FieldId) {
        self.field_id = id
    }

    fn set_field_name(&mut self, name: String) {
        self.field_name = name
    }

    fn set_field_type(&mut self, dtype: DCTypeDefinition) {
        self.field_type = dtype;
        self.has_default_value = false;
        self.default_value = vec![];
    }

    fn set_default_value(&mut self, value: Vec<u8>) {
        self.default_value = value;
        self.has_default_value = true;
        self.default_value_stale = false;
    }

    fn set_bogus_field(&mut self, is_bogus: bool) {
        self.bogus_field = is_bogus
    }

    fn set_parent_struct(&mut self, parent: Arc<Mutex<DCStruct>>) {
        assert!(self.dclass.is_none());
        self._struct = Some(parent);
    }

    fn set_parent_dclass(&mut self, parent: Arc<Mutex<DClass>>) {
        assert!(self._struct.is_none());
        self.dclass = Some(parent);
    }

    #[inline(always)]
    fn has_default_value(&self) -> bool {
        self.has_default_value
    }

    fn validate_ranges(&self, packed_data: &Datagram) -> bool {
        todo!()
    }

    #[inline(always)]
    fn is_bogus_field(&self) -> bool {
        self.bogus_field
    }

    #[inline(always)]
    fn is_required(&self) -> bool {
        has_keyword!(self, "required")
    }

    #[inline(always)]
    fn is_broadcast(&self) -> bool {
        has_keyword!(self, "broadcast")
    }

    #[inline(always)]
    fn is_ram(&self) -> bool {
        has_keyword!(self, "ram")
    }

    #[inline(always)]
    fn is_db(&self) -> bool {
        has_keyword!(self, "db")
    }

    #[inline(always)]
    fn is_clsend(&self) -> bool {
        has_keyword!(self, "clsend")
    }

    #[inline(always)]
    fn is_clrecv(&self) -> bool {
        has_keyword!(self, "clrecv")
    }

    #[inline(always)]
    fn is_ownsend(&self) -> bool {
        has_keyword!(self, "ownsend")
    }

    #[inline(always)]
    fn is_ownrecv(&self) -> bool {
        has_keyword!(self, "ownrecv")
    }

    #[inline(always)]
    fn is_airecv(&self) -> bool {
        has_keyword!(self, "airecv")
    }
}
