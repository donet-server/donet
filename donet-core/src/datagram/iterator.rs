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

//! Provides structure for iterating over network packets (datagrams).

use super::datagram::Datagram;
use crate::datagram::byte_order as endianness;
use crate::globals;
use crate::protocol::*;
use std::mem;
use strum::IntoEnumIterator;
use thiserror::Error;

/// Custom error type for [`DatagramIterator`].
#[derive(Debug, Error, PartialEq)]
pub enum IteratorError {
    /// This error kind is returned when attempting
    /// to read past the end of a datagram.
    #[error("end of file")]
    EndOfFile,
    /// This error kind is returned when attempting to read a field
    /// where the provided field value (if of numerical type) or
    /// field length (if of array type) is outside the provided
    /// sane range for the given field type.
    #[error("field constraint violation")]
    FieldConstraintViolation,
}

/// Utility for iterating value by value of a datagram message.
#[derive(Debug)]
pub struct DatagramIterator {
    datagram: Datagram,
    index: usize,
}

/// Create a new [`DatagramIterator`] from a [`Datagram`].
impl From<Datagram> for DatagramIterator {
    fn from(value: Datagram) -> Self {
        Self {
            datagram: value,
            index: 0_usize,
        }
    }
}

impl DatagramIterator {
    pub fn check_read_length(&mut self, bytes: globals::DgSizeTag) -> Result<(), IteratorError> {
        let new_index: globals::DgSizeTag = self.index as globals::DgSizeTag + bytes;

        if new_index > self.datagram.size() {
            // FIXME: error!("The DatagramIterator tried to read past the end of the buffer!");
            return Err(IteratorError::EndOfFile);
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
    pub fn skip(&mut self, bytes: globals::DgSizeTag) -> Result<(), IteratorError> {
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
    pub fn read_msg_type(&mut self) -> Protocol {
        let start_index: usize = self.index;

        self.index = 1
            + usize::from(self.read_recipient_count()) * mem::size_of::<globals::Channel>()
            + mem::size_of::<globals::Channel>(); // seek message type

        let msg_type: globals::MsgType = self.read_u16(); // read message type
        self.index = start_index; // do not advance dgi index

        for message in Protocol::iter() {
            let msg_id: globals::MsgType = message.into();
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
    use super::*;
    use crate::datagram::datagram::DatagramError;

    #[test]
    #[rustfmt::skip]
    fn dgi_read_integers() {
        let mut dg: Datagram = Datagram::default();
        let mut results: Vec<Result<(), DatagramError>> = vec![];

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

        let mut dgi: DatagramIterator = dg.into();

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
        let mut dg: Datagram = Datagram::default();
        let mut results: Vec<Result<(), DatagramError>> = vec![];

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

        let mut dgi: DatagramIterator = dg.into();

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
        let mut dg: Datagram = Datagram::default();

        let test_msg_types: Vec<Protocol> = vec![
            Protocol::MDAddChannel,
            Protocol::CAAddInterest,
            Protocol::SSDeleteAIObjects,
        ];

        for m_type in &test_msg_types {
            let res: Result<(), DatagramError> = dg.add_u16((*m_type).into());
            if res.is_err() {
                panic!("{:#?}", res.unwrap_err());
            }
        }
        let mut dgi: DatagramIterator = dg.into();

        for m_type in &test_msg_types {
            let read_msg_type: globals::MsgType = dgi.read_u16();
            assert_eq!(read_msg_type, globals::MsgType::from(*m_type));
        }
    }
}
