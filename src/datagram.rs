// DONET SOFTWARE
// Copyright (c) 2023, Donet Authors.

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

#[path = "message_types.rs"]
mod messages;

#[allow(dead_code)] // FIXME: Remove once project matures
mod datagram {
    use std::vec::Vec;
    use std::result::Result; // not to be confused with std::io::Result

    type DgSize = u16;
    const DG_SIZE_MAX: DgSize = u16::MAX;

    // All possible errors that can be returned within Datagram's implementation.
    pub enum DgError {
        DatagramOverflow,
    }

    pub type DgResult = Result<(), DgError>;

    pub struct Datagram {
        buffer: Vec<u8>,
    }

    impl Datagram {
        pub fn new() -> Datagram {
            Datagram {
                buffer: Vec::new(),
            }
        }

        // Checks if we can add `length` number of bytes to the datagram.
        fn check_add_length(&self, length: DgSize) -> DgResult {
            let new_offset: usize = self.buffer.len() + usize::from(length);
            
            if new_offset > DG_SIZE_MAX.into() {
                // TODO: log error with more information
                return Err(DgError::DatagramOverflow);
            }
            return Ok(());
        }

        // Adds an 8-bit integer to the datagram that is guaranteed
        // to be one of the values 0x00 (false) or 0x01 (true).
        fn add_bool(&mut self, v: bool) -> DgResult {
            let mut res: DgResult = self.check_add_length(1);
            if res.is_err() {
                return res;
            }
            if v {
                res = self.add_u8(1);
            } else {
                res = self.add_u8(0);
            }
            return res;
        }

        fn add_u8(&mut self, v: u8) -> DgResult {
            let res: DgResult = self.check_add_length(1);
            if res.is_err() {
                return res;
            }
            self.buffer.push(v);
            return Ok(());
        }
    }

    //pub struct DatagramIterator {

    //}
}
