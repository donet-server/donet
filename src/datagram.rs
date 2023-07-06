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

// Detect system endianness (byte order)
mod endianness {
    #[cfg(target_endian = "big")]
    pub fn swap_le_16(v: u16) -> u16 {
        return (v & 0x00ff) << 8 |
               (v & 0xff00) >> 8;
    }

    #[cfg(target_endian = "big")]
    pub fn swap_le_32(v: u32) -> u32 {
        return (v & 0x000000ff) << 24 |
               (v & 0x0000ff00) <<  8 |
               (v & 0x00ff0000) >>  8 |
               (v & 0xff000000) >> 24;
    }

    #[cfg(target_endian = "big")]
    pub fn swap_le_64(v: u64) -> u64 {
        return (v & 0x00000000000000ff) << 56 |
               (v & 0x000000000000ff00) << 40 |
               (v & 0x0000000000ff0000) << 24 |
               (v & 0x00000000ff000000) <<  8 |
               (v & 0x000000ff00000000) >>  8 |
               (v & 0x0000ff0000000000) >> 24 |
               (v & 0x00ff000000000000) >> 40 |
               (v & 0xff00000000000000) >> 56;
    }

    #[cfg(target_endian = "little")]
    pub fn swap_le_16(v: u16) -> u16 {
        return v; // no need to swap bytes
    }

    #[cfg(target_endian = "little")]
    pub fn swap_le_32(v: u32) -> u32 {
        return v;
    }

    #[cfg(target_endian = "little")]
    pub fn swap_le_64(v: u64) -> u64 {
        return v;
    }
}

#[allow(dead_code)] // FIXME: Remove once project matures
mod datagram {
    use crate::datagram::endianness;
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

        fn add_multiple_bytes(&mut self, bytes: DgSize, ) -> DgResult {
            let res: DgResult = self.check_add_length(bytes);
            if res.is_err() {
                return res;
            }

            return Ok(());
        }

        // Adds an unsigned 8-bit integer to the datagram that is
        // guaranteed to be one of the values 0x00 (false) or 0x01 (true).
        pub fn add_bool(&mut self, v: bool) -> DgResult {
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

        // Adds an unsigned 8-bit integer value to the datagram.
        pub fn add_u8(&mut self, v: u8) -> DgResult {
            let res: DgResult = self.check_add_length(1);
            if res.is_err() {
                return res;
            }
            self.buffer.push(v);
            return Ok(());
        }

        pub fn add_u16(&mut self, mut v: u16) -> DgResult {
            let res: DgResult = self.check_add_length(2);
            if res.is_err() {
                return res;
            }
            v = endianness::swap_le_16(v);
            // FIXME: There is definitely a simpler way to do this.
            // Masking each byte and shifting it to the first byte,
            // then casting it as a u8 to represent one byte.
            self.buffer.push((v & 0xff00) as u8);
            self.buffer.push(((v & 0x00ff) << 8) as u8);
            return Ok(());
        }

        pub fn add_u32(&mut self, mut v: u32) -> DgResult {
            let res: DgResult = self.check_add_length(4);
            if res.is_err() {
                return res;
            }
            v = endianness::swap_le_32(v);
            self.buffer.push((v & 0xff000000) as u8);
            self.buffer.push(((v & 0x00ff0000) << 8) as u8);
            self.buffer.push(((v & 0x0000ff00) << 16) as u8);
            self.buffer.push(((v & 0x000000ff) << 24) as u8);
            return Ok(());
        }

        pub fn add_u64(&mut self, mut v: u64) -> DgResult {
            let res: DgResult = self.check_add_length(8);
            if res.is_err() {
                return res;
            }
            v = endianness::swap_le_64(v);
            self.buffer.push((v & 0xff00000000000000) as u8);
            self.buffer.push(((v & 0x00ff000000000000) << 8) as u8);
            self.buffer.push(((v & 0x0000ff0000000000) << 16) as u8);
            self.buffer.push(((v & 0x000000ff00000000) << 24) as u8);
            self.buffer.push(((v & 0x00000000ff000000) << 32) as u8);
            self.buffer.push(((v & 0x0000000000ff0000) << 40) as u8);
            self.buffer.push(((v & 0x000000000000ff00) << 48) as u8);
            self.buffer.push(((v & 0x00000000000000ff) << 56) as u8);
            return Ok(());
        }
    }

    //pub struct DatagramIterator {

    //}
}
