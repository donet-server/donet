// DONET SOFTWARE
// Copyright (c) 2023, DoNet Authors.

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

#[allow(dead_code)] // FIXME: Remove once project matures
pub mod type_aliases {
    use std::mem;

    // Type Definitions
    pub type Channel = u64;
    pub type DoId = u32;
    pub type Zone = u32;

    // Type Limits
    pub const CHANNEL_MAX: Channel = u64::MAX;
    pub const DOID_MAX: DoId = u32::MAX;
    pub const ZONE_MAX: Zone = u32::MAX;
    pub const ZONE_BITS: usize = 8 * mem::size_of::<Zone>();

    // DoId Constants
    pub const INVALID_DOID: DoId = 0;

    // Channel Constants
    pub const INVALID_CHANNEL: Channel = 0;
    pub const CONTROL_CHANNEL: Channel = 1;
    pub const BCHAN_CLIENTS: Channel = 10;
    pub const BCHAN_STATESERVERS: Channel = 12;
    pub const BCHAN_DBSERVERS: Channel = 13;
}
