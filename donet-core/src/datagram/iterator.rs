/*
    This file is part of Donet.

    Copyright © 2024 Max Rodriguez <me@maxrdz.com>

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

use super::datagram::{Datagram, DatagramError};
use crate::datagram::byte_order as endianness;
use crate::globals::*;
use crate::protocol::*;
use std::mem;
use std::string::FromUtf8Error;
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
    #[error("could not convert bytes to UTF-8")]
    Utf8Error(FromUtf8Error),
    #[error("invalid read; {0}")]
    InvalidRead(&'static str),
    #[error("datagram error")]
    DatagramError(DatagramError),
}

impl From<IteratorError> for std::io::Error {
    fn from(value: IteratorError) -> std::io::Error {
        std::io::Error::new(
            match &value {
                IteratorError::EndOfFile => std::io::ErrorKind::UnexpectedEof,
                _ => std::io::ErrorKind::Other,
            },
            value.to_string(),
        )
    }
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
            index: 0,
        }
    }
}

impl DatagramIterator {
    pub fn check_read_length(&mut self, bytes: usize) -> Result<(), IteratorError> {
        let new_index: usize = self.index + bytes;

        if new_index > self.datagram.size() {
            return Err(IteratorError::EndOfFile);
        }
        Ok(())
    }

    /// Returns the value of `self.index`, which is in bytes.
    #[inline]
    pub fn tell(&mut self) -> usize {
        self.index
    }

    /// Manually sets the `index` position.
    #[inline]
    pub fn seek(&mut self, index: usize) {
        self.index = index
    }

    /// Increments the `index` by `bytes` length.
    /// Returns DgError.DatagramIteratorEOF if it's past the end of the buffer.
    pub fn skip(&mut self, bytes: usize) -> Result<(), IteratorError> {
        self.check_read_length(bytes)?;
        self.index += bytes;
        Ok(())
    }

    /// Returns the number of unread bytes left in the datagram
    pub fn get_remaining(&mut self) -> usize {
        self.datagram.size() - self.index
    }

    /// Reads the next number of bytes in the datagram.
    pub fn read_data(&mut self, bytes: usize) -> Result<Vec<u8>, IteratorError> {
        self.check_read_length(bytes)?;
        let data: Vec<u8> = self.datagram.get_data();

        let mut new_data: Vec<u8> = vec![];
        let read_end: usize = self.index + bytes;

        for item in data.iter().take(read_end).skip(self.index) {
            new_data.push(*item);
        }
        self.index += bytes;

        Ok(new_data)
    }

    pub fn read_u8(&mut self) -> Result<u8, IteratorError> {
        self.check_read_length(1)?;
        let data: Vec<u8> = self.datagram.get_data();

        match data.get(self.index) {
            Some(v) => {
                self.index += 1; // bytes
                Ok(*v)
            }
            None => {
                return Err(IteratorError::EndOfFile);
            }
        }
    }

    pub fn read_u16(&mut self) -> Result<u16, IteratorError> {
        self.check_read_length(2)?;
        let data: Vec<u8> = self.datagram.get_data();

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

        Ok(endianness::swap_le_16(value))
    }

    pub fn read_u32(&mut self) -> Result<u32, IteratorError> {
        self.check_read_length(4)?;
        let data: Vec<u8> = self.datagram.get_data();

        let value: u32 = (data[self.index] as u32)
            | ((data[self.index + 1] as u32) << 8)
            | ((data[self.index + 2] as u32) << 16)
            | ((data[self.index + 3] as u32) << 24);

        self.index += 4;
        Ok(endianness::swap_le_32(value))
    }

    pub fn read_u64(&mut self) -> Result<u64, IteratorError> {
        self.check_read_length(8)?;
        let data: Vec<u8> = self.datagram.get_data();

        let value: u64 = (data[self.index] as u64)
            | ((data[self.index + 1] as u64) << 8)
            | ((data[self.index + 2] as u64) << 16)
            | ((data[self.index + 3] as u64) << 24)
            | ((data[self.index + 4] as u64) << 32)
            | ((data[self.index + 5] as u64) << 40)
            | ((data[self.index + 6] as u64) << 48)
            | ((data[self.index + 7] as u64) << 56);

        self.index += 8;
        Ok(endianness::swap_le_64(value))
    }

    // Signed integer aliases, same read operation.
    #[inline]
    pub fn read_i8(&mut self) -> Result<i8, IteratorError> {
        self.read_u8().map(|v| v as i8)
    }

    #[inline]
    pub fn read_i16(&mut self) -> Result<i16, IteratorError> {
        self.read_u16().map(|v| v as i16)
    }

    #[inline]
    pub fn read_i32(&mut self) -> Result<i32, IteratorError> {
        self.read_u32().map(|v| v as i32)
    }

    #[inline]
    pub fn read_i64(&mut self) -> Result<i64, IteratorError> {
        self.read_u64().map(|v| v as i64)
    }

    /// 32-bit IEEE 754 floating point in native endianness.
    #[inline]
    pub fn read_f32(&mut self) -> Result<f32, IteratorError> {
        self.read_u32().map(|v| v as f32)
    }

    /// 64-bit IEEE 754 floating point in native endianness.
    #[inline]
    pub fn read_f64(&mut self) -> Result<f64, IteratorError> {
        self.read_u64().map(|v| v as f64)
    }

    #[inline]
    pub fn read_bool(&mut self) -> Result<bool, IteratorError> {
        Ok(self.read_u8()? == 1)
    }

    /// Attempts to read a `String` data type from the datagram
    /// as a **UTF-8 string**. Returns a [`String`] if OK.
    ///
    /// If the string type payload is not of UTF-8 format, a
    /// [`IteratorError::Utf8Error`] variant will be returned.
    pub fn read_string(&mut self) -> Result<String, IteratorError> {
        let str_len: DgSizeTag = self.read_size()?;

        let str_bytes: Vec<u8> = self.read_data(usize::from(str_len))?;

        let utf8_str: String = match String::from_utf8(str_bytes) {
            Ok(data) => data,
            Err(e) => {
                return Err(IteratorError::Utf8Error(e));
            }
        };
        Ok(utf8_str)
    }

    #[inline]
    pub fn read_size(&mut self) -> Result<DgSizeTag, IteratorError> {
        self.read_u16().map(|v| v.into())
    }

    #[inline]
    pub fn read_channel(&mut self) -> Result<Channel, IteratorError> {
        self.read_u64().map(|v| v.into())
    }

    #[inline]
    pub fn read_doid(&mut self) -> Result<DoId, IteratorError> {
        self.read_u32().map(|v| v.into())
    }

    #[inline]
    pub fn read_zone(&mut self) -> Result<Zone, IteratorError> {
        self.read_u32().map(|v| v.into())
    }

    /// Reads a `blob` data type and returns a [`Datagram`].
    pub fn read_datagram(&mut self) -> Result<Datagram, IteratorError> {
        let dg_size: DgSizeTag = self.read_size()?;

        let dg_payload: Vec<u8> = self.read_data(usize::from(dg_size))?;

        let mut new_dg: Datagram = Datagram::default();

        if let Err(e) = new_dg.add_data(dg_payload) {
            return Err(IteratorError::DatagramError(e));
        }
        Ok(new_dg)
    }

    /// Get the recipient count in a datagram message.
    ///
    /// Alias of [`Datagram::read_u8`].
    #[inline(always)]
    pub fn read_recipient_count(&mut self) -> Result<u8, IteratorError> {
        self.read_u8()
    }

    /// Returns the datagram's message type as a [`Protocol`] variant.
    pub fn read_msg_type(&mut self) -> Result<Protocol, IteratorError> {
        let msg_type: MsgType = self.read_u16()?; // read message type

        for message in Protocol::iter() {
            let msg_id: MsgType = message.into();
            if msg_type == msg_id {
                return Ok(message);
            }
        }
        Err(IteratorError::InvalidRead(
            "Tried to read an invalid message type.",
        ))
    }

    /// Get the recipient count in a datagram message.
    /// Does not advance the index.
    pub fn peek_recipient_count(&mut self) -> Result<u8, IteratorError> {
        let start_index: usize = self.index;
        let value: u8 = self.read_u8()?;
        self.index = start_index;
        Ok(value)
    }

    /// Returns the datagram's message type. Does not advance the index.
    /// Useful for if index needs to be saved or if next field isn't msg type.
    /// If iterating through a fresh datagram, use [`Self::read_msg_type`].
    pub fn peek_msg_type(&mut self) -> Result<Protocol, IteratorError> {
        let start_index: usize = self.index;

        self.index = 1
            + usize::from(self.peek_recipient_count()?) * mem::size_of::<Channel>()
            + mem::size_of::<Channel>(); // seek message type

        let msg_type: MsgType = self.read_u16()?; // read message type
        self.index = start_index; // do not advance dgi index

        for message in Protocol::iter() {
            let msg_id: MsgType = message.into();

            if msg_type == msg_id {
                return Ok(message);
            }
        }
        Err(IteratorError::InvalidRead(
            "Tried to read an invalid message type.",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::datagram::datagram::DatagramError;

    #[test]
    #[rustfmt::skip]
    fn dgi_read_integers() -> Result<(), IteratorError> {
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
        let res_tag: DgSizeTag = dgi.read_u16()?;
        // Unsigned integers
        let res_u8: u8 = dgi.read_u8()?;
        let res_u16: u16 = dgi.read_u16()?;
        let res_u32: u32 = dgi.read_u32()?;
        let res_u64: u64 = dgi.read_u64()?;
        // Signed integers
        let res_i8: i8 = dgi.read_i8()?;
        let res_i16: i16 = dgi.read_i16()?;
        let res_i32: i32 = dgi.read_i32()?;
        let res_i64: i64 = dgi.read_i64()?;
        // Floating point
        let res_f32: f32 = dgi.read_f32()?;
        let res_f64: f64 = dgi.read_f64()?;

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
        Ok(())
    }

    #[test]
    fn dgi_read_dc_types() -> Result<(), IteratorError> {
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
        let res_size: DgSizeTag = dgi.read_size()?;
        // Boolean
        let res_bool_false: bool = dgi.read_bool()?;
        let res_bool_true: bool = dgi.read_bool()?;
        // DC Types
        let res_channel: Channel = dgi.read_channel()?;
        let res_doid: DoId = dgi.read_doid()?;
        let res_zone: Zone = dgi.read_zone()?;

        assert_eq!(res_size, 18_u16); // DC blob size tag
        assert_eq!(res_bool_false, false);
        assert_eq!(res_bool_true, true);
        assert_eq!(res_channel, 0_u64);
        assert_eq!(res_doid, 0_u32);
        assert_eq!(res_zone, 0_u32);
        assert_eq!(dgi.get_remaining(), 0); // iterator should be exhausted
        Ok(())
    }

    #[test]
    fn dgi_read_message_type() -> Result<(), IteratorError> {
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
            let read_msg_type: MsgType = dgi.read_u16()?;
            assert_eq!(read_msg_type, MsgType::from(*m_type));
        }
        Ok(())
    }
}
