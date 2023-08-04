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

#[path = "results.rs"] mod results;
#[path = "types.rs"] mod type_aliases;

// Detect system endianness (byte order)
pub mod endianness {
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
pub mod datagram {
    use log::{error};
    use super::results::results as res;
    use super::type_aliases::type_aliases as types;
    use super::endianness;
    use std::vec::Vec;

    type DgSize = u16;
    const DG_SIZE_MAX: DgSize = u16::MAX;

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
        fn check_add_length(&self, length: DgSize) -> res::DgResult {
            let new_offset: usize = self.buffer.len() + usize::from(length);
            
            if new_offset > DG_SIZE_MAX.into() {
                error!("Tried to add data to the datagram past its maximum size!");
                return Err(res::DgError::DatagramOverflow);
            }
            return Ok(());
        }

        // Adds an unsigned 8-bit integer to the datagram that is
        // guaranteed to be one of the values 0x00 (false) or 0x01 (true).
        pub fn add_bool(&mut self, v: bool) -> res::DgResult {
            self.check_add_length(1)?;
            if v {
                self.add_u8(1)?;
            } else {
                self.add_u8(0)?;
            }
            return Ok(());
        }
 
        // Adds an unsigned 8-bit integer value to the datagram.
        pub fn add_u8(&mut self, v: u8) -> res::DgResult {
            self.check_add_length(1)?;
            self.buffer.push(v);
            return Ok(());
        }

        pub fn add_u16(&mut self, mut v: u16) -> res::DgResult {
            self.check_add_length(2)?;
            v = endianness::swap_le_16(v);
            // NOTE: I feel like there is a simpler way to do this.
            // Masking each byte and shifting it to the first byte,
            // then casting it as a u8 to represent one byte.
            self.buffer.push((v & 0xff00) as u8);
            self.buffer.push(((v & 0x00ff) << 8) as u8);
            return Ok(());
        }

        pub fn add_u32(&mut self, mut v: u32) -> res::DgResult {
            self.check_add_length(4)?;
            v = endianness::swap_le_32(v);
            self.buffer.push((v & 0xff000000) as u8);
            self.buffer.push(((v & 0x00ff0000) << 8) as u8);
            self.buffer.push(((v & 0x0000ff00) << 16) as u8);
            self.buffer.push(((v & 0x000000ff) << 24) as u8);
            return Ok(());
        }

        pub fn add_u64(&mut self, mut v: u64) -> res::DgResult {
            self.check_add_length(8)?;
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

        // signed integer aliases. same bitwise operations.
        pub fn add_i8(&mut self, v: i8) -> res::DgResult {
            return self.add_u8(v as u8);
        }

        pub fn add_i16(&mut self, v: i16) -> res::DgResult {
            return self.add_u16(v as u16);
        }

        pub fn add_i32(&mut self, v: i32) -> res::DgResult { 
            return self.add_u32(v as u32);
        }

        pub fn add_i64(&mut self, v: i64) -> res::DgResult { 
            return self.add_u64(v as u64);
        }
        
        // 32-bit IEEE 754 floating point. same bitwise operations.
        pub fn add_f32(&mut self, v: f32) -> res::DgResult {
            return self.add_u32(v as u32);
        }

        // 64-bit IEEE 754 floating point. same bitwise operations.
        pub fn add_f64(&mut self, v: f64) -> res::DgResult {
            return self.add_u64(v as u64);
        }

        // Adds a Datagram / Field length tag to the end of the datagram.
        pub fn add_size(&mut self, v: DgSize) -> res::DgResult {
            return self.add_u16(v as u16);
        }

        // Adds a 64-bit channel ID to the end of the datagram.
        pub fn add_channel(&mut self, v: types::Channel) -> res::DgResult {
            return self.add_u64(v as u64);
        }

        // Adds a 32-bit Distributed Object ID to the end of the datagram.
        pub fn add_doid(&mut self, v: types::DoId) -> res::DgResult {
            return self.add_u32(v as u32);
        }

        // Adds a 32-bit zone ID to the end of the datagram.
        pub fn add_zone(&mut self, v: types::Zone) -> res::DgResult {
            return self.add_u32(v as u32);
        }

        // Added for convenience, but also better performance
        // than adding the parent and the zone separately.
        pub fn add_location(&mut self, parent: types::DoId, zone: types::Zone) -> res::DgResult {
            self.add_u32(parent as u32)?;
            return self.add_u32(zone as u32);
        }

        // Adds raw bytes to the datagram via an unsigned 8-bit integer vector.
        // NOTE: not to be confused with add_blob(), which adds a dclass blob to the datagram.
        pub fn add_data(&mut self, mut v: Vec<u8>) -> res::DgResult {
            if v.len() > DG_SIZE_MAX.into() { // check input to avoid panic at .try_into() below
                return Err(res::DgError::DatagramOverflow); 
            }
            self.check_add_length(v.len().try_into().unwrap())?;
            self.buffer.append(&mut v);
            return Ok(());
        }

        // Appends another datagram's binary data to this datagram.
        pub fn add_datagram(&mut self, dg: Datagram) -> res::DgResult {
            let mut dg_buffer: Vec<u8> = dg.buffer;
            
            if dg_buffer.len() > DG_SIZE_MAX.into() {
                // Technically should not happen as the datagram given should
                // keep its buffer under the max dg size, but we should still handle
                // this error to avoid a panic at self.check_add_length().
                return Err(res::DgError::DatagramOverflow);
            }
            self.check_add_length(dg_buffer.len().try_into().unwrap())?;
            self.buffer.append(&mut dg_buffer);
            return Ok(());
        }

        // Adds a dclass string value to the end of the datagram.
        // A 16-bit length tag prefix with the string's size in bytes is added.
        pub fn add_string(&mut self, v: &str) -> res::DgResult {
            if v.len() > DG_SIZE_MAX.into() {
                // The string is too big to be described with a 16-bit length tag.
                return Err(res::DgError::DatagramOverflow);
            }
            // Add string length to the datagram
            self.add_u16(v.len().try_into().unwrap())?;

            // convert the string into a byte array, as a vector
            let str_bytes: &mut Vec<u8> = &mut v.as_bytes().to_vec();
            
            // make sure the byte array won't overflow the datagram
            self.check_add_length(str_bytes.len().try_into().unwrap())?;
            self.buffer.append(str_bytes);
            return Ok(());
        }

        // Adds a dclass blob value (binary data) to the end of the datagram.
        // A 16-bit length tag prefix with the blob's size in bytes is added.
        pub fn add_blob(&mut self, mut v: Vec<u8>) -> res::DgResult {
            // add blob size in bytes
            self.add_size(v.len().try_into().unwrap())?;
            // manually check add length before appending byte array
            self.check_add_length(v.len().try_into().unwrap())?;
            self.buffer.append(&mut v);
            return Ok(());
        }

        // TODO: add_buffer() method to reserve space in datagram buffer.

        // Appends a generic header for messages that are to be routed to
        // one or more role instances within the server cluster.
        // Use this method to avoid repetitive code for every internal message.
        //
        // The header is formatted as shown below:
        //     (recipients: u8, recipients: Vec<Channel>, sender: Channel, message_type: u16)
        //
        pub fn add_server_header(&mut self, to: Vec<types::Channel>,
                                 from: types::Channel, msg_type: u16) -> res::DgResult {
            // Add recipient(s) count
            self.add_u8(to.len().try_into().unwrap())?;

            for recipient in to { // append each recipient in vector given
                self.add_channel(recipient)?;
            }
            self.add_channel(from)?;
            self.add_u16(msg_type)?;
            return Ok(());
        }

        // Appends a control header, which is very similar to a server header,
        // but it always has only one recipient, which is the control channel,
        // and does not require a sender (or 'from') channel to be provided.
        pub fn add_control_header(&mut self, msg_type: u16) -> res::DgResult {
            self.add_u8(1)?;
            self.add_channel(types::CONTROL_CHANNEL)?;
            self.add_u16(msg_type)?;
            return Ok(());
        }

        pub fn size(&mut self) -> DgSize {
            return self.buffer.len().try_into().unwrap();
        }

        pub fn get_data(&mut self) -> Vec<u8> {
            // we can't give out ownership of our vector,
            // so a copy of the vector is made instead
            let mut vec_copy: Vec<u8> = vec![];
            for byte in &self.buffer {
                // dereference the borrowed 'byte'
                vec_copy.push(*byte);
            }
            return vec_copy;
        }
    }

    // Utility for iterating value by value of a datagram message.
    pub struct DatagramIterator {
        datagram: Datagram,
        offset: usize,
    }

    impl DatagramIterator {
        pub fn new(&self, dg: Datagram) -> DatagramIterator {
            DatagramIterator {
                datagram: dg,
                offset: 0 as usize,
            }
        }

        pub fn check_read_length(&mut self, bytes: DgSize) -> res::DgResult {
            let new_offset: DgSize = self.offset as DgSize + bytes;

            if new_offset > self.datagram.size() {
                error!("The DatagramIterator tried to read past the end of the buffer!");
                return Err(res::DgError::DatagramIteratorEOF);
            }
            return Ok(());
        }

        // Returns the value of `buffer_offset` in bytes.
        pub fn tell(&mut self) -> DgSize {
            return self.offset as DgSize;
        }

        // Manually sets the buffer_offset position.
        pub fn seek(&mut self, to: DgSize) -> () {
            self.offset = to as usize;
        }

        // Increments the buffer_offset by `bytes` length.
        // Returns DgError.DatagramIteratorEOF if it's past the end of the buffer.
        pub fn skip(&mut self, bytes: DgSize) -> res::DgResult {
            self.check_read_length(bytes)?;
            self.offset += bytes as usize;
            return Ok(());
        }

        // Returns the number of unread bytes left in the datagram
        pub fn get_remaining(&mut self) -> DgSize {
            return self.datagram.size() - self.offset as DgSize;
        }

        // Reads the next number of bytes in the datagram.
        pub fn read_data(&mut self, bytes: DgSize) -> Vec<u8> {
            let data: Vec<u8> = self.datagram.get_data();

            let mut new_data: Vec<u8> = vec![];
            let read_end: usize = self.offset + bytes as usize;

            for n in self.offset..read_end {
                new_data.push(data[n]);
            }
            self.offset += bytes as usize;
            return new_data;
        }

        pub fn read_u8(&mut self) -> u8 {
            let data: Vec<u8> = self.datagram.get_data();
            let value: u8 = data[self.offset];
            self.offset += 1; // bytes
            return value;
        }

        pub fn read_u16(&mut self) -> u16 {
            let data: Vec<u8> = self.datagram.get_data();

            // bitwise operations to concatenate two u8's into one u16.
            // graphical explanation:
            //      a0   (byte 1)           b0   (byte 2)
            //      11010001                00100111
            //
            //      [ a1 = a0 as u16 ]      [ b1 = b0 as u16 ]
            //      00000000 11010001       00000000 00100111
            //
            //      [ a2 = a1 << 8 ]             v v v v
            //      11010001 00000000
            //
            //              00000000 00100111
            //          OR  11010001 00000000
            //
            //              11010001 00100111  (u16, 2 bytes)
            //
            //  After, we use the swap_le_xx() function to make sure the bytes
            //  are swapped to the native system byte endianness.
            //
            let value: u16 = ((data[self.offset] as u16) << 8) | data[self.offset + 1] as u16;
            self.offset += 1;
            return endianness::swap_le_16(value);
        }

        pub fn read_u32(&mut self) -> u32 {
            let data: Vec<u8> = self.datagram.get_data();
            let value: u32 = ((data[self.offset] as u32) << 24) |
                             ((data[self.offset + 1] as u32) << 16) |
                             ((data[self.offset + 2] as u32) << 8) |
                               data[self.offset + 3] as u32;
            self.offset += 4;
            return endianness::swap_le_32(value);
        }

        pub fn read_u64(&mut self) -> u64 {
            let data: Vec<u8> = self.datagram.get_data();
            let value: u64 = ((data[self.offset] as u64) << 56) |
                             ((data[self.offset + 1] as u64) << 48) |
                             ((data[self.offset + 2] as u64) << 40) |
                             ((data[self.offset + 3] as u64) << 32) |
                             ((data[self.offset + 4] as u64) << 24) |
                             ((data[self.offset + 5] as u64) << 16) |
                             ((data[self.offset + 6] as u64) << 8) |
                               data[self.offset + 7] as u64;
            self.offset += 8;
            return endianness::swap_le_64(value);
        }

        // Signed integer aliases, same read operation.
        pub fn read_i8(&mut self) -> i8 {
            return self.read_u8() as i8;
        }

        pub fn read_i16(&mut self) -> i16 {
            return self.read_u16() as i16;
        }

        pub fn read_i32(&mut self) -> i32 {
            return self.read_u32() as i32;
        }

        pub fn read_i64(&mut self) -> i64 {
            return self.read_u64() as i64;
        }

        // 32-bit IEEE 754 floating point in native endianness.
        pub fn read_f32(&mut self) -> f32 {
            return self.read_u32() as f32;
        }

        // 64-bit IEEE 754 floating point in native endianness.
        pub fn read_f64(&mut self) -> f64 {
            return self.read_u64() as f64;
        }

        pub fn read_bool(&mut self) -> bool {
            let data: u8 = self.read_u8();
            return if data == 1 { true } else { false }
        }

        pub fn read_size(&mut self) -> DgSize {
            return self.read_u16() as DgSize;
        }

        pub fn read_channel(&mut self) -> types::Channel {
            return self.read_u64() as types::Channel;
        }

        pub fn read_doid(&mut self) -> types::DoId {
            return self.read_u32() as types::DoId;
        }

        pub fn read_zone(&mut self) -> types::Zone {
            return self.read_u32() as types::Zone;
        }
    }
}
