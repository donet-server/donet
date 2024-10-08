/*
    This file is part of Donet.

    Copyright © 2024 Max Rodriguez

    Donet is free software; you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License,
    as published by the Free Software Foundation, either version 3
    of the License, or (at your option) any later version.

    Donet is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public
    License along with Donet. If not, see <https://www.gnu.org/licenses/>.
*/

//! Provides structures to write network datagrams and tools
//! for iterating over datagram data.

use crate::datagram::byte_order as endianness;
use crate::globals;
use std::mem;
use std::vec::Vec;
use strum::IntoEnumIterator;

/// Representation of a new network message (datagram) to be sent.
#[derive(Default, Debug, Clone)]
pub struct Datagram {
    buffer: Vec<u8>,
    index: usize,
}

impl Datagram {
    /// Creates a new empty Datagram. Alias for the `Default` implementation.
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if we can add `length` number of bytes to the datagram.
    fn check_add_length(&mut self, length: globals::DgSizeTag) -> globals::DgResult {
        let new_index: usize = self.index + usize::from(length);

        if new_index > globals::DG_SIZE_MAX.into() {
            // FIXME: error!("Tried to add data to the datagram past its maximum size!");
            return Err(globals::DgError::DatagramOverflow);
        }
        Ok(())
    }

    /// Adds an unsigned 8-bit integer to the datagram that is
    /// guaranteed to be one of the values 0x00 (false) or 0x01 (true).
    pub fn add_bool(&mut self, v: bool) -> globals::DgResult {
        self.check_add_length(1)?;
        if v {
            self.add_u8(1)?;
        } else {
            self.add_u8(0)?;
        }
        Ok(())
    }

    /// Adds an unsigned 8-bit integer value to the datagram.
    pub fn add_u8(&mut self, v: u8) -> globals::DgResult {
        self.check_add_length(1)?;
        self.buffer.push(v);
        self.index += 1;
        Ok(())
    }

    pub fn add_u16(&mut self, mut v: u16) -> globals::DgResult {
        self.check_add_length(2)?;
        v = endianness::swap_le_16(v);
        // NOTE: I feel like there is a simpler way to do this.
        // Masking each byte and shifting it to the first byte,
        // then casting it as a u8 to represent one byte.
        self.buffer.push((v & 0x00ff) as u8);
        self.buffer.push(((v & 0xff00) >> 8) as u8);
        self.index += 2;
        Ok(())
    }

    pub fn add_u32(&mut self, mut v: u32) -> globals::DgResult {
        self.check_add_length(4)?;
        v = endianness::swap_le_32(v);
        self.buffer.push((v & 0x000000ff) as u8);
        self.buffer.push(((v & 0x0000ff00) >> 8) as u8);
        self.buffer.push(((v & 0x00ff0000) >> 16) as u8);
        self.buffer.push(((v & 0xff000000) >> 24) as u8);
        self.index += 4;
        Ok(())
    }

    pub fn add_u64(&mut self, mut v: u64) -> globals::DgResult {
        self.check_add_length(8)?;
        v = endianness::swap_le_64(v);
        self.buffer.push((v & 0x00000000000000ff) as u8);
        self.buffer.push(((v & 0x000000000000ff00) >> 8) as u8);
        self.buffer.push(((v & 0x0000000000ff0000) >> 16) as u8);
        self.buffer.push(((v & 0x00000000ff000000) >> 24) as u8);
        self.buffer.push(((v & 0x000000ff00000000) >> 32) as u8);
        self.buffer.push(((v & 0x0000ff0000000000) >> 40) as u8);
        self.buffer.push(((v & 0x00ff000000000000) >> 48) as u8);
        self.buffer.push(((v & 0xff00000000000000) >> 56) as u8);
        self.index += 8;
        Ok(())
    }

    // signed integer aliases. same bitwise operations.
    pub fn add_i8(&mut self, v: i8) -> globals::DgResult {
        self.add_u8(v as u8)
    }

    pub fn add_i16(&mut self, v: i16) -> globals::DgResult {
        self.add_u16(v as u16)
    }

    pub fn add_i32(&mut self, v: i32) -> globals::DgResult {
        self.add_u32(v as u32)
    }

    pub fn add_i64(&mut self, v: i64) -> globals::DgResult {
        self.add_u64(v as u64)
    }

    /// 32-bit IEEE 754 floating point. same bitwise operations.
    pub fn add_f32(&mut self, v: f32) -> globals::DgResult {
        self.add_u32(v as u32)
    }

    /// 64-bit IEEE 754 floating point. same bitwise operations.
    pub fn add_f64(&mut self, v: f64) -> globals::DgResult {
        self.add_u64(v as u64)
    }

    /// Adds a Datagram / Field length tag to the end of the datagram.
    pub fn add_size(&mut self, v: globals::DgSizeTag) -> globals::DgResult {
        self.add_u16(v)
    }

    /// Adds a 64-bit channel ID to the end of the datagram.
    pub fn add_channel(&mut self, v: globals::Channel) -> globals::DgResult {
        self.add_u64(v)
    }

    /// Adds a 32-bit Distributed Object ID to the end of the datagram.
    pub fn add_doid(&mut self, v: globals::DoId) -> globals::DgResult {
        self.add_u32(v)
    }

    /// Adds a 32-bit zone ID to the end of the datagram.
    pub fn add_zone(&mut self, v: globals::Zone) -> globals::DgResult {
        self.add_u32(v)
    }

    /// Added for convenience, rather than adding the parent and the zone separately.
    pub fn add_location(&mut self, parent: globals::DoId, zone: globals::Zone) -> globals::DgResult {
        self.add_u32(parent)?;
        self.add_u32(zone)
    }

    /// Adds raw bytes to the datagram via an unsigned 8-bit integer vector.
    /// NOTE: not to be confused with add_blob(), which adds a dclass blob to the datagram.
    pub fn add_data(&mut self, mut v: Vec<u8>) -> globals::DgResult {
        if v.len() > globals::DG_SIZE_MAX.into() {
            // check input to avoid panic at .try_into() below
            return Err(globals::DgError::DatagramOverflow);
        }
        self.check_add_length(v.len().try_into().unwrap())?;
        self.buffer.append(&mut v);
        self.index += v.len();
        Ok(())
    }

    /// Appends another datagram's binary data to this datagram.
    pub fn add_datagram(&mut self, dg: &mut Datagram) -> globals::DgResult {
        let dg_buffer: Vec<u8> = dg.get_data();

        if dg_buffer.len() > globals::DG_SIZE_MAX.into() {
            // Technically should not happen as the datagram given should
            // keep its buffer under the max dg size, but we should still handle
            // this error to avoid a panic at self.check_add_length().
            return Err(globals::DgError::DatagramOverflow);
        }
        self.add_data(dg_buffer)
    }

    /// Adds a dclass string value to the end of the datagram.
    /// A 16-bit length tag prefix with the string's size in bytes is added.
    pub fn add_string(&mut self, v: &str) -> globals::DgResult {
        if v.len() > globals::DG_SIZE_MAX.into() {
            // The string is too big to be described with a 16-bit length tag.
            return Err(globals::DgError::DatagramOverflow);
        }
        // Add string length to the datagram
        self.add_u16(v.len().try_into().unwrap())?;

        // convert the string into a byte array, as a vector
        let str_bytes: &mut Vec<u8> = &mut v.as_bytes().to_vec();

        // make sure the byte array won't overflow the datagram
        self.check_add_length(str_bytes.len().try_into().unwrap())?;
        self.buffer.append(str_bytes);
        self.index += v.len();
        Ok(())
    }

    /// Adds a dclass blob value (binary data) to the end of the datagram.
    /// A 16-bit length tag prefix with the blob's size in bytes is added.
    pub fn add_blob(&mut self, mut v: Vec<u8>) -> globals::DgResult {
        // add blob size in bytes
        self.add_size(v.len().try_into().unwrap())?;
        // manually check add length before appending byte array
        self.check_add_length(v.len().try_into().unwrap())?;
        self.buffer.append(&mut v);
        self.index += v.len();
        Ok(())
    }

    /// Reserves an amount of bytes in the datagram buffer.
    pub fn add_buffer(&mut self, bytes: globals::DgSizeTag) -> globals::DgBufferResult {
        self.check_add_length(bytes)?;
        // get start length (before push)
        let start: globals::DgSizeTag = self.index as globals::DgSizeTag;
        for _n in 1..bytes {
            self.buffer.push(0_u8);
        }
        self.index += usize::from(bytes);
        Ok(start)
    }

    /// Appends a generic header for messages that are to be routed to
    /// one or more role instances within the server cluster.
    /// Use this method to avoid repetitive code for every internal message.
    ///
    /// The header is formatted as shown below:
    ///     (recipients: `u8`, recipients: `Vec<Channel>`, sender: `Channel`, message_type: `u16`)
    ///
    pub fn add_server_header(
        &mut self,
        to: Vec<globals::Channel>,
        from: globals::Channel,
        msg_type: globals::MsgType,
    ) -> globals::DgResult {
        // Add recipient(s) count
        self.add_u8(to.len().try_into().unwrap())?;

        for recipient in to {
            // append each recipient in vector given
            self.add_channel(recipient)?;
        }
        self.add_channel(from)?;
        self.add_u16(msg_type)?;
        Ok(())
    }

    /// Appends a control header, which is very similar to a server header,
    /// but it always has only one recipient, which is the control channel,
    /// and does not require a sender (or 'from') channel to be provided.
    pub fn add_control_header(&mut self, msg_type: globals::MsgType) -> globals::DgResult {
        self.add_u8(1)?;
        self.add_channel(globals::CONTROL_CHANNEL)?;
        self.add_u16(msg_type)?;
        Ok(())
    }

    pub fn size(&mut self) -> globals::DgSizeTag {
        self.buffer.len().try_into().unwrap()
    }

    pub fn get_data(&mut self) -> Vec<u8> {
        // we can't give out ownership of our vector,
        // so a copy of the vector is made instead
        let mut vec_copy: Vec<u8> = vec![];
        for byte in &self.buffer {
            // dereference the borrowed 'byte'
            vec_copy.push(*byte);
        }
        vec_copy
    }
}

/// Utility for iterating value by value of a datagram message.
#[derive(Debug)]
pub struct DatagramIterator {
    datagram: Datagram,
    index: usize,
}

impl DatagramIterator {
    pub fn new(dg: Datagram) -> Self {
        DatagramIterator {
            datagram: dg,
            index: 0_usize,
        }
    }

    pub fn check_read_length(&mut self, bytes: globals::DgSizeTag) -> globals::DgResult {
        let new_index: globals::DgSizeTag = self.index as globals::DgSizeTag + bytes;

        if new_index > self.datagram.size() {
            // FIXME: error!("The DatagramIterator tried to read past the end of the buffer!");
            return Err(globals::DgError::DatagramIteratorEOF);
        }
        Ok(())
    }

    /// Returns the value of `self.index` in bytes.
    pub fn tell(&mut self) -> globals::DgSizeTag {
        self.index as globals::DgSizeTag
    }

    /// Manually sets the buffer_offset position.
    pub fn seek(&mut self, to: globals::DgSizeTag) {
        self.index = to as usize;
    }

    /// Increments the buffer_offset by `bytes` length.
    /// Returns DgError.DatagramIteratorEOF if it's past the end of the buffer.
    pub fn skip(&mut self, bytes: globals::DgSizeTag) -> globals::DgResult {
        self.check_read_length(bytes)?;
        self.index += bytes as usize;
        Ok(())
    }

    /// Returns the number of unread bytes left in the datagram
    pub fn get_remaining(&mut self) -> globals::DgSizeTag {
        self.datagram.size() - self.index as globals::DgSizeTag
    }

    /// Reads the next number of bytes in the datagram.
    pub fn read_data(&mut self, bytes: globals::DgSizeTag) -> Vec<u8> {
        let data: Vec<u8> = self.datagram.get_data();

        let mut new_data: Vec<u8> = vec![];
        let read_end: usize = self.index + bytes as usize;

        for item in data.iter().take(read_end).skip(self.index) {
            new_data.push(*item);
        }
        self.index += bytes as usize;
        new_data
    }

    pub fn read_u8(&mut self) -> u8 {
        let data: Vec<u8> = self.datagram.get_data();
        if self.check_read_length(1_u16).is_err() {
            panic!("Tried to read past the end of a datagram message!");
        }
        let value: u8 = data[self.index];
        self.index += 1; // bytes
        value
    }

    pub fn read_u16(&mut self) -> u16 {
        let data: Vec<u8> = self.datagram.get_data();
        if self.check_read_length(2_u16).is_err() {
            panic!("Tried to read past the end of a datagram message!");
        }
        // bitwise operations to concatenate two u8's into one u16.
        // graphical explanation:
        //      a0   (byte 1; 0x28)     b0   (byte 2; 0x23)
        //      00101000                01000110
        //
        //      NOTE: Turns out rust casts these in big-endian
        //
        //      [ a1 = a0 as u16 ]      [ b1 = b0 as u16 ]
        //      00000000 00101000       00000000 01000110
        //
        //          v v v v v           [ b2 = b1 << 8 ]
        //                              01000110 00000000
        //
        //              00000000 00101000 = a1
        //          OR  01000110 00000000 = b2
        //
        //              01000110 00101000  (u16, 2 bytes; 0x2328; 9000 decimal)
        //
        //  After, we use the swap_le_xx() function to make sure the bytes
        //  are swapped to the native system byte endianness.
        //
        let value: u16 = (data[self.index] as u16) | ((data[self.index + 1] as u16) << 8);
        self.index += 2;
        endianness::swap_le_16(value)
    }

    pub fn read_u32(&mut self) -> u32 {
        let data: Vec<u8> = self.datagram.get_data();
        if self.check_read_length(4_u16).is_err() {
            panic!("Tried to read past the end of a datagram message!");
        }
        let value: u32 = (data[self.index] as u32)
            | ((data[self.index + 1] as u32) << 8)
            | ((data[self.index + 2] as u32) << 16)
            | ((data[self.index + 3] as u32) << 24);
        self.index += 4;
        endianness::swap_le_32(value)
    }

    pub fn read_u64(&mut self) -> u64 {
        let data: Vec<u8> = self.datagram.get_data();
        if self.check_read_length(8_u16).is_err() {
            panic!("Tried to read past the end of a datagram message!");
        }
        let value: u64 = (data[self.index] as u64)
            | ((data[self.index + 1] as u64) << 8)
            | ((data[self.index + 2] as u64) << 16)
            | ((data[self.index + 3] as u64) << 24)
            | ((data[self.index + 4] as u64) << 32)
            | ((data[self.index + 5] as u64) << 40)
            | ((data[self.index + 6] as u64) << 48)
            | ((data[self.index + 7] as u64) << 56);
        self.index += 8;
        endianness::swap_le_64(value)
    }

    // Signed integer aliases, same read operation.
    pub fn read_i8(&mut self) -> i8 {
        self.read_u8() as i8
    }

    pub fn read_i16(&mut self) -> i16 {
        self.read_u16() as i16
    }

    pub fn read_i32(&mut self) -> i32 {
        self.read_u32() as i32
    }

    pub fn read_i64(&mut self) -> i64 {
        self.read_u64() as i64
    }

    /// 32-bit IEEE 754 floating point in native endianness.
    pub fn read_f32(&mut self) -> f32 {
        self.read_u32() as f32
    }

    /// 64-bit IEEE 754 floating point in native endianness.
    pub fn read_f64(&mut self) -> f64 {
        self.read_u64() as f64
    }

    pub fn read_bool(&mut self) -> bool {
        let data: u8 = self.read_u8();
        data == 1
    }

    pub fn read_size(&mut self) -> globals::DgSizeTag {
        self.read_u16() as globals::DgSizeTag
    }

    pub fn read_channel(&mut self) -> globals::Channel {
        self.read_u64() as globals::Channel
    }

    pub fn read_doid(&mut self) -> globals::DoId {
        self.read_u32() as globals::DoId
    }

    pub fn read_zone(&mut self) -> globals::Zone {
        self.read_u32() as globals::Zone
    }

    /// Get the recipient count in a datagram message.
    /// Does not advance the DatagramIterator index.
    pub fn read_recipient_count(&mut self) -> u8 {
        if self.datagram.size() == 0 {
            // FIXME: error!("Cannot read from an empty datagram!");
            // FIXME: Throw error instead of panic here.
            panic!("Tried to read from an empty datagram.");
        }
        let start_index: usize = self.index;
        let value: u8 = self.read_u8();
        self.index = start_index;
        value
    }

    /// Returns the datagram's message type. Does not advance the index.
    /// Useful for if index needs to be saved or if next field isn't msg type.
    /// If iterating through a fresh datagram, use read_u16.
    pub fn read_msg_type(&mut self) -> globals::Protocol {
        let start_index: usize = self.index;

        self.index = 1
            + usize::from(self.read_recipient_count()) * mem::size_of::<globals::Channel>()
            + mem::size_of::<globals::Channel>(); // seek message type

        let msg_type: globals::MsgType = self.read_u16(); // read message type
        self.index = start_index; // do not advance dgi index

        for message in globals::Protocol::iter() {
            let msg_id: globals::MsgType = globals::msg_type(message);
            if msg_type == msg_id {
                return message;
            }
        }
        // FIXME: Throw error instead of panic here.
        panic!("Tried to read an invalid message type from datagram.");
    }
}

#[cfg(test)]
mod unit_testing {
    use crate::datagram::datagram;
    use crate::globals;
    use crate::globals::{msg_type, Protocol};

    // ----------- Datagram ------------ //
    #[test]
    fn dg_add_boolean() {
        let mut dg: datagram::Datagram = datagram::Datagram::default();
        let mut res: globals::DgResult = dg.add_bool(true);
        match res {
            Ok(_) => {}
            Err(err) => panic!("add_bool(true) error: {:?}", err),
        }
        res = dg.add_bool(false);
        match res {
            Ok(_) => {}
            Err(err) => panic!("add_bool(false) error: {:?}", err),
        }
    }

    #[test]
    fn dg_add_integers_and_types() {
        // A bit repetitive, but we need coverage on all of these methods.
        let mut dg: datagram::Datagram = datagram::Datagram::default();
        let mut results: Vec<globals::DgResult> = vec![];
        // Signed integers
        results.push(dg.add_i8(i8::MAX));
        results.push(dg.add_i16(i16::MAX));
        results.push(dg.add_i32(i32::MAX));
        results.push(dg.add_i64(i64::MAX));
        // Unsigned integers
        results.push(dg.add_u8(u8::MAX));
        results.push(dg.add_u16(u16::MAX));
        results.push(dg.add_u32(u32::MAX));
        results.push(dg.add_u64(u64::MAX));
        // 32-bit/64-bit floats
        results.push(dg.add_f32(f32::MAX));
        results.push(dg.add_f64(f64::MAX));
        // Types (aliases)
        results.push(dg.add_size(globals::DG_SIZE_MAX));
        results.push(dg.add_channel(globals::CHANNEL_MAX));
        results.push(dg.add_doid(globals::DOID_MAX));
        results.push(dg.add_zone(globals::ZONE_MAX));
        results.push(dg.add_location(globals::DOID_MAX, globals::ZONE_MAX));
        results.push(dg.add_string("TEST")); // 16-bit length prefix + # of chars
        results.push(dg.add_blob(vec![u8::MAX, u8::MAX])); // same prefix as above
        results.push(dg.add_data(vec![u8::MAX, u8::MAX, u8::MAX, u8::MAX]));

        for dg_res in &results {
            assert!(dg_res.is_ok());
        }
        let dg_size: globals::DgSizeTag = dg.size();
        let dg_buffer: Vec<u8> = dg.get_data();

        assert_eq!(dg_buffer.len() as u16, dg_size); // verify buffer length
        assert_eq!(dg_size, 82); // total in bytes
    }

    #[test]
    #[rustfmt::skip]
    fn dg_add_datagram() {
        let mut dg: datagram::Datagram = datagram::Datagram::default();
        let mut dg_2: datagram::Datagram = datagram::Datagram::new();

        assert!(dg.add_channel(globals::CHANNEL_MAX).is_ok());
        assert!(dg_2.add_blob(vec![0, 125, u8::MAX]).is_ok());
        assert!(dg.add_datagram(&mut dg_2).is_ok());

        let dg_size: globals::DgSizeTag = dg.size();
        let dg_buffer: Vec<u8> = dg.get_data();

        assert_eq!(dg_buffer.len() as u16, dg_size);
        assert_eq!(dg_buffer, vec![
            u8::MAX, u8::MAX, u8::MAX, u8::MAX,
            u8::MAX, u8::MAX, u8::MAX, u8::MAX,
            3, 0, 0, 125, u8::MAX,
        ]);
    }

    #[test]
    #[rustfmt::skip]
    fn dg_add_message_headers() {
        let mut dg: datagram::Datagram = datagram::Datagram::default();
        let mut results: Vec<globals::DgResult> = vec![];

        results.push(dg.add_server_header(
            vec![globals::CHANNEL_MAX], // recipients
            0, // sender
            msg_type(Protocol::MDAddChannel), // msg type
        ));

        results.push(dg.add_control_header(msg_type(Protocol::MDAddChannel)));

        for dg_res in &results {
            assert!(dg_res.is_ok());
        }
        let dg_size: globals::DgSizeTag = dg.size();
        let dg_buffer: Vec<u8> = dg.get_data();

        assert_eq!(dg_buffer.len() as u16, dg_size);
        assert_eq!(dg_buffer, vec![
            1, u8::MAX, u8::MAX, u8::MAX, u8::MAX, // recipients
            u8::MAX, u8::MAX, u8::MAX, u8::MAX,
            0, 0, 0, 0, 0, 0, 0, 0, // sender
            40, 35, // message type (9000; 0x2823, or 40, 35)
            1, 1, 0, 0, 0, 0, 0, 0, 0, // recipients (control)
            40, 35, // message type
        ]);
    }

    #[test]
    fn datagram_overflow_test() {
        let mut dg: datagram::Datagram = datagram::Datagram::default();
        let res_1: globals::DgBufferResult = dg.add_buffer(globals::DG_SIZE_MAX);

        assert!(!res_1.is_err(), "Could not append 2^16 bytes to datagram buffer.");
        assert_eq!(res_1.unwrap(), 0, "add_buffer() didn't return start of reserve.");
        assert_eq!(
            dg.size() + 1,
            globals::DG_SIZE_MAX,
            "Datagram didn't add 2^16 bytes to the buffer."
        );

        let res_2: globals::DgResult = dg.add_u16(0);
        assert!(
            res_2.is_err(),
            "Datagram overflow occurred, but did not throw an error."
        );

        assert_eq!(
            res_2.unwrap_err(),
            globals::DgError::DatagramOverflow,
            "Datagram overflow occurred, but failed to respond with DgError::DatagramOverflow."
        );
    }

    #[test]
    #[rustfmt::skip]
    fn dgi_read_integers() {
        let mut dg: datagram::Datagram = datagram::Datagram::default();
        let mut results: Vec<globals::DgResult> = vec![];

        results.push(dg.add_blob(vec![
            u8::MAX, // 8-bits
            u8::MAX, u8::MAX, // 16-bits
            u8::MAX, u8::MAX, u8::MAX, u8::MAX, // 32-bits
            u8::MAX, u8::MAX, u8::MAX, u8::MAX, u8::MAX, u8::MAX, u8::MAX, u8::MAX, // 64-bits
            u8::MAX, // 8-bits
            0, 0x80_u8, // 16-bits (i16::MIN)
            u8::MAX, u8::MAX, u8::MAX, u8::MAX, // 32-bits
            0, 0, 0, 0, 0, 0, 0, 0x80_u8, // 64-bits (i64::MIN)
            0, 0, 0, 0, // 32-bits
            0, 0, 0, 0, 0, 0, 0, 0 // 64-bits
        ]));
        for dg_res in &results {
            assert!(dg_res.is_ok());
        }
        results.clear(); // clear results from dg setup

        let mut dgi: datagram::DatagramIterator = datagram::DatagramIterator::new(dg);

        // Read blob type length
        let res_tag: globals::DgSizeTag = dgi.read_u16();
        // Unsigned integers
        let res_u8: u8 = dgi.read_u8();
        let res_u16: u16 = dgi.read_u16();
        let res_u32: u32 = dgi.read_u32();
        let res_u64: u64 = dgi.read_u64();
        // Signed integers
        let res_i8: i8 = dgi.read_i8();
        let res_i16: i16 = dgi.read_i16();
        let res_i32: i32 = dgi.read_i32();
        let res_i64: i64 = dgi.read_i64();
        // Floating point
        let res_f32: f32 = dgi.read_f32();
        let res_f64: f64 = dgi.read_f64();

        assert_eq!(res_tag, 42_u16); // DC blob size tag
        assert_eq!(res_u8, u8::MAX);
        assert_eq!(res_u16, u16::MAX);
        assert_eq!(res_u32, u32::MAX);
        assert_eq!(res_u64, u64::MAX);
        assert_eq!(res_i8, -1); // 0b11111111 is -1 base 10 :)
        assert_eq!(res_i16, i16::MIN);
        assert_eq!(res_i32, -1);
        assert_eq!(res_i64, i64::MIN);
        assert_eq!(res_f32, 0.0);
        assert_eq!(res_f64, 0.0);
        assert_eq!(dgi.get_remaining(), 0); // iterator should be exhausted
    }

    #[test]
    fn dgi_read_dc_types() {
        let mut dg: datagram::Datagram = datagram::Datagram::default();
        let mut results: Vec<globals::DgResult> = vec![];

        results.push(dg.add_blob(vec![
            0x00_u8, // boolean false
            0x01_u8, // boolean true
            0, 0, 0, 0, 0, 0, 0, 0, // channel
            0, 0, 0, 0, 0, 0, 0, 0, // location (doid + zone)
        ]));
        for dg_res in &results {
            assert!(dg_res.is_ok());
        }
        results.clear(); // clear results from dg setup

        let mut dgi: datagram::DatagramIterator = datagram::DatagramIterator::new(dg);

        // Size Tag
        let res_size: globals::DgSizeTag = dgi.read_size();
        // Boolean
        let res_bool_false: bool = dgi.read_bool();
        let res_bool_true: bool = dgi.read_bool();
        // DC Types
        let res_channel: globals::Channel = dgi.read_channel();
        let res_doid: globals::DoId = dgi.read_doid();
        let res_zone: globals::Zone = dgi.read_zone();

        assert_eq!(res_size, 18_u16); // DC blob size tag
        assert_eq!(res_bool_false, false);
        assert_eq!(res_bool_true, true);
        assert_eq!(res_channel, 0_u64);
        assert_eq!(res_doid, 0_u32);
        assert_eq!(res_zone, 0_u32);
        assert_eq!(dgi.get_remaining(), 0); // iterator should be exhausted
    }

    #[test]
    fn dgi_read_message_type() {
        let mut dg: datagram::Datagram = datagram::Datagram::default();

        let test_msg_types: Vec<Protocol> = vec![
            Protocol::MDAddChannel,
            Protocol::CAAddInterest,
            Protocol::SSDeleteAIObjects,
        ];

        for m_type in &test_msg_types {
            let res: globals::DgResult = dg.add_u16(msg_type(*m_type));
            if res.is_err() {
                panic!("{:#?}", res.unwrap_err());
            }
        }
        let mut dgi: datagram::DatagramIterator = datagram::DatagramIterator::new(dg);

        for m_type in &test_msg_types {
            let read_msg_type: globals::MsgType = dgi.read_u16();
            assert_eq!(read_msg_type, msg_type(*m_type));
        }
    }
}
