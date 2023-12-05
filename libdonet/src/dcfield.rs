// DONET SOFTWARE
// Copyright (c) 2023, Donet Authors.
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

use crate::dclass::DClass;
use crate::dcstruct::DCStruct;
use crate::globals;
use std::sync::{Arc, Mutex}; // thread safe

// --------- Field ---------- //

pub struct DCField {
    class: Option<Arc<Mutex<DClass>>>,
    _struct: Option<Arc<Mutex<DCStruct>>>,
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
    fn generate_hash(&mut self);
    fn set_field_id(&mut self, id: globals::FieldId);
    fn set_field_name(&mut self, name: String);
    fn set_parent_struct(&mut self, parent: Arc<Mutex<DCStruct>>);
    fn set_parent_dclass(&mut self, parent: Arc<Mutex<DClass>>);
}
