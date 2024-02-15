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

use crate::dckeyword::{DCKeywordList, DCKeywordListInterface, IdentifyKeyword};
use crate::dclass::DClass;
use crate::dcstruct::DCStruct;
use crate::globals;
use crate::hashgen::DCHashGenerator;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct DCField {
    parent: DCKeywordList,
    class: Option<Arc<Mutex<DClass>>>,
    _struct: Option<Arc<Mutex<DCStruct>>>, // needs '_' due to reserved keyword
    field_name: String,
    field_id: globals::FieldId,
    parent_is_dclass: bool,
    default_value_stale: bool,
    has_default_value: bool,
    default_value: Vec<u8>, // stored as byte array
    bogus_field: bool,
}

pub trait DCFieldInterface {
    fn new(name: &str, id: globals::FieldId) -> Self;
    fn generate_hash(&self, hashgen: &mut DCHashGenerator);

    fn set_field_id(&mut self, id: globals::FieldId);
    fn set_field_name(&mut self, name: String);

    fn set_parent_struct(&mut self, parent: Arc<Mutex<DCStruct>>);
    fn set_parent_dclass(&mut self, parent: Arc<Mutex<DClass>>);

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

impl DCFieldInterface for DCField {
    fn new(name: &str, id: globals::FieldId) -> Self {
        Self {
            parent: DCKeywordList::new(),
            class: None,
            _struct: None,
            field_name: name.to_owned(),
            field_id: id,
            parent_is_dclass: false,
            default_value_stale: false,
            has_default_value: false,
            default_value: vec![],
            bogus_field: false,
        }
    }

    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        self.dckeywordlist_generate_hash(hashgen);
        // TODO!
    }

    #[inline(always)]
    fn set_field_id(&mut self, id: globals::FieldId) {
        self.field_id = id
    }

    #[inline(always)]
    fn set_field_name(&mut self, name: String) {
        self.field_name = name
    }

    fn set_parent_struct(&mut self, parent: Arc<Mutex<DCStruct>>) {
        todo!()
    }

    fn set_parent_dclass(&mut self, parent: Arc<Mutex<DClass>>) {
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
        &self.parent
    }
}
