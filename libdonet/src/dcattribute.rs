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

use crate::dcfield::DCField;

/// A DC Attribute Field is a type of DC Field which can be found
/// in DC Structs and Distributed Classes.
///
/// Unlike the Panda source, structure elements are called attributes,
/// instead of parameters, as it raises confusion with DC Atomic Field's
/// elements, which are a simpler form of Panda's DC Parameters, as they
/// do not carry DC Keywords, but their corresponding DC Atomic Field does.
#[derive(Debug)]
pub struct DCAttributeField {
    _dcattributefield_parent: DCField,
}

/// See issue #22.
impl std::ops::Deref for DCAttributeField {
    type Target = DCField;
    fn deref(&self) -> &Self::Target {
        &self._dcattributefield_parent
    }
}
