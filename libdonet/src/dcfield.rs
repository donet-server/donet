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
use crate::dckeyword::{DCKeywordList, DCKeywordListInterface, IdentifyKeyword};
use crate::dclass::DClass;
use crate::dcstruct::DCStruct;
use crate::dctype::{DCTypeDefinition, DCTypeDefinitionInterface};
use crate::globals;
use crate::hashgen::DCHashGenerator;
use std::sync::{Arc, Mutex};

/// A field of a Distributed Class. The DCField struct is a base for
/// struct and dclass fields. In the DC language, there are three types
/// of field declarations, which are: parameter, atomic, and molecular.
#[derive(Debug)]
pub struct DCField {
    _dcfield_parent: DCKeywordList,
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

pub trait DCFieldInterface {
    fn new(name: &str, id: globals::FieldId) -> Self;
    fn dcfield_generate_hash(&self, hashgen: &mut DCHashGenerator);

    fn get_field_id(&self) -> globals::FieldId;
    fn get_dclass(&self) -> Arc<Mutex<DClass>>;

    fn set_field_id(&mut self, id: globals::FieldId);
    fn set_field_name(&mut self, name: String);
    fn set_default_value(&mut self, value: Vec<u8>);

    fn set_parent_struct(&mut self, parent: Arc<Mutex<DCStruct>>);
    fn set_parent_dclass(&mut self, parent: Arc<Mutex<DClass>>);

    fn has_default_value(&self) -> bool;
    fn validate_ranges(&self, packed_data: &Datagram) -> bool;

    // Inline functions for Panda historical keywords
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

impl DCFieldInterface for DCField {
    fn new(name: &str, id: globals::FieldId) -> Self {
        Self {
            _dcfield_parent: DCKeywordList::new(),
            dclass: None,
            _struct: None,
            field_name: name.to_owned(),
            field_id: id,
            field_type: DCTypeDefinition::new(),
            parent_is_dclass: false,
            default_value_stale: false,
            has_default_value: false,
            default_value: vec![],
            bogus_field: false,
        }
    }

    fn dcfield_generate_hash(&self, hashgen: &mut DCHashGenerator) {
        self.dckeywordlist_generate_hash(hashgen);
        // TODO!
    }

    fn get_field_id(&self) -> globals::FieldId {
        todo!()
    }

    fn get_dclass(&self) -> Arc<Mutex<DClass>> {
        todo!()
    }

    #[inline(always)]
    fn set_field_id(&mut self, id: globals::FieldId) {
        self.field_id = id
    }

    #[inline(always)]
    fn set_field_name(&mut self, name: String) {
        self.field_name = name
    }

    fn set_default_value(&mut self, value: Vec<u8>) {
        todo!()
    }

    fn set_parent_struct(&mut self, parent: Arc<Mutex<DCStruct>>) {
        todo!()
    }

    fn set_parent_dclass(&mut self, parent: Arc<Mutex<DClass>>) {
        todo!()
    }

    fn has_default_value(&self) -> bool {
        todo!()
    }

    fn validate_ranges(&self, packed_data: &Datagram) -> bool {
        todo!()
    }

    #[inline(always)]
    fn is_required(&self) -> bool {
        self.has_keyword(IdentifyKeyword::ByName("required".to_owned()))
    }

    #[inline(always)]
    fn is_broadcast(&self) -> bool {
        self.has_keyword(IdentifyKeyword::ByName("broadcast".to_owned()))
    }

    #[inline(always)]
    fn is_ram(&self) -> bool {
        self.has_keyword(IdentifyKeyword::ByName("ram".to_owned()))
    }

    #[inline(always)]
    fn is_db(&self) -> bool {
        self.has_keyword(IdentifyKeyword::ByName("db".to_owned()))
    }

    #[inline(always)]
    fn is_clsend(&self) -> bool {
        self.has_keyword(IdentifyKeyword::ByName("clsend".to_owned()))
    }

    #[inline(always)]
    fn is_clrecv(&self) -> bool {
        self.has_keyword(IdentifyKeyword::ByName("clrecv".to_owned()))
    }

    #[inline(always)]
    fn is_ownsend(&self) -> bool {
        self.has_keyword(IdentifyKeyword::ByName("ownsend".to_owned()))
    }

    #[inline(always)]
    fn is_ownrecv(&self) -> bool {
        self.has_keyword(IdentifyKeyword::ByName("ownrecv".to_owned()))
    }

    #[inline(always)]
    fn is_airecv(&self) -> bool {
        self.has_keyword(IdentifyKeyword::ByName("airecv".to_owned()))
    }
}

/// 'Fake' inheritance of DCKeywordList object.
/// See issue #22.
impl std::ops::Deref for DCField {
    type Target = DCKeywordList;
    fn deref(&self) -> &Self::Target {
        &self._dcfield_parent
    }
}
