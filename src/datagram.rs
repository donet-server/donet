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

#[path = "types.rs"]
mod type_aliases;

//use std::io::{Error, Result};

#[allow(dead_code)] // FIXME: Remove once project matures
mod datagram {
    type DgSize = u16;

    const DG_SIZE_MAX: DgSize = u16::MAX;

    pub struct Datagram {
        buffer: *mut u8,
        buffer_cap: u16,
        buffer_offset: u16,
    }

    impl Datagram {
        //fn check_add_length(&self, length: DgSize) -> Result<()> {

        //}
    }

    //pub struct DatagramIterator {

    //}
}
