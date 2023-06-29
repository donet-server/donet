// PANDANET SOFTWARE
// Copyright (c) 2023, PandaNet Authors.

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
mod type_aliases {
    use std::mem;

    // Type Definitions
    type Channel = u64;
    type DoId = u32;
    type Zone = u32;

    // Type Limits
    const CHANNEL_MAX: Channel = u64::MAX;
    const DOID_MAX: DoId = u32::MAX;
    const ZONE_MAX: Zone = u32::MAX;
    const ZONE_BITS: usize = 8 * mem::size_of::<Zone>();

    // DoId Constants
    const INVALID_DOID: DoId = 0;

    // Channel Constants
    const INVALID_CHANNEL: Channel = 0;
    const CONTROL_CHANNEL: Channel = 1;
    const BCHAN_CLIENTS: Channel = 10;
    const BCHAN_STATESERVERS: Channel = 12;
    const BCHAN_DBSERVERS: Channel = 13;
}
