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

//! Provides structure to write network packets (datagrams).

use crate::datagram::byte_order as endianness;
use crate::globals;
use anyhow::Result;
use thiserror::Error;

/// Custom error type for [`Datagram`].
#[derive(Debug, Error, PartialEq)]
pub enum DatagramError {
    #[error("datagram overflow; {0}")]
    DatagramOverflow(&'static str),
}

/// Representation of a new network message (datagram) to be sent.
#[derive(Debug, Default, Clone)]
pub struct Datagram {
    buffer: Vec<u8>,
    index: usize,
}

/// Appends another datagram's binary data to this datagram.
impl std::ops::Add for Datagram {
    type Output = Result<Datagram, DatagramError>;

    fn add(mut self, rhs: Self) -> Self::Output {
        let dg_buffer: Vec<u8> = rhs.get_data();

        if dg_buffer.len() > globals::DG_SIZE_MAX.into() {
            // Technically should not happen as the datagram given should
            // keep its buffer under the max dg size, but we should still handle
            // this error to avoid a panic at self.check_add_length().
            return Err(DatagramError::DatagramOverflow(
                "Datagram given will overflow left-hand-side datagram.",
            ));
        }
        self.add_data(dg_buffer)?;

        Ok(self)
    }
}

impl Datagram {
    /// Checks if we can add `length` number of bytes to the datagram.
    fn check_add_length(&mut self, length: globals::DgSizeTag) -> Result<(), DatagramError> {
        let new_index: usize = self.index + usize::from(length);

        if new_index > globals::DG_SIZE_MAX.into() {
            return Err(DatagramError::DatagramOverflow(
                "Tried to add data to the datagram past its maximum size!",
            ));
        }
        Ok(())
    }

    /// Adds an unsigned 8-bit integer to the datagram that is
    /// guaranteed to be one of the values 0x00 (false) or 0x01 (true).
    pub fn add_bool(&mut self, v: bool) -> Result<(), DatagramError> {
        self.check_add_length(1)?;
        if v {
            self.add_u8(1)?;
        } else {
            self.add_u8(0)?;
        }
        Ok(())
    }

    /// Adds an unsigned 8-bit integer value to the datagram.
    pub fn add_u8(&mut self, v: u8) -> Result<(), DatagramError> {
        self.check_add_length(1)?;
        self.buffer.push(v);
        self.index += 1;
        Ok(())
    }

    pub fn add_u16(&mut self, mut v: u16) -> Result<(), DatagramError> {
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

    pub fn add_u32(&mut self, mut v: u32) -> Result<(), DatagramError> {
        self.check_add_length(4)?;
        v = endianness::swap_le_32(v);
        self.buffer.push((v & 0x000000ff) as u8);
        self.buffer.push(((v & 0x0000ff00) >> 8) as u8);
        self.buffer.push(((v & 0x00ff0000) >> 16) as u8);
        self.buffer.push(((v & 0xff000000) >> 24) as u8);
        self.index += 4;
        Ok(())
    }

    pub fn add_u64(&mut self, mut v: u64) -> Result<(), DatagramError> {
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
    #[inline(always)]
    pub fn add_i8(&mut self, v: i8) -> Result<(), DatagramError> {
        self.add_u8(v as u8)
    }

    #[inline(always)]
    pub fn add_i16(&mut self, v: i16) -> Result<(), DatagramError> {
        self.add_u16(v as u16)
    }

    #[inline(always)]
    pub fn add_i32(&mut self, v: i32) -> Result<(), DatagramError> {
        self.add_u32(v as u32)
    }

    #[inline(always)]
    pub fn add_i64(&mut self, v: i64) -> Result<(), DatagramError> {
        self.add_u64(v as u64)
    }

    /// 32-bit IEEE 754 floating point. same bitwise operations.
    #[inline(always)]
    pub fn add_f32(&mut self, v: f32) -> Result<(), DatagramError> {
        self.add_u32(v as u32)
    }

    /// 64-bit IEEE 754 floating point. same bitwise operations.
    #[inline(always)]
    pub fn add_f64(&mut self, v: f64) -> Result<(), DatagramError> {
        self.add_u64(v as u64)
    }

    /// Adds a Datagram / Field length tag to the end of the datagram.
    #[inline(always)]
    pub fn add_size(&mut self, v: globals::DgSizeTag) -> Result<(), DatagramError> {
        self.add_u16(v)
    }

    /// Adds a 64-bit channel ID to the end of the datagram.
    #[inline(always)]
    pub fn add_channel(&mut self, v: globals::Channel) -> Result<(), DatagramError> {
        self.add_u64(v)
    }

    /// Adds a 32-bit Distributed Object ID to the end of the datagram.
    #[inline(always)]
    pub fn add_doid(&mut self, v: globals::DoId) -> Result<(), DatagramError> {
        self.add_u32(v)
    }

    /// Adds a 32-bit zone ID to the end of the datagram.
    #[inline(always)]
    pub fn add_zone(&mut self, v: globals::Zone) -> Result<(), DatagramError> {
        self.add_u32(v)
    }

    /// Added for convenience, rather than adding the parent and the zone separately.
    #[inline(always)]
    pub fn add_location(&mut self, parent: globals::DoId, zone: globals::Zone) -> Result<(), DatagramError> {
        self.add_u32(parent)?;
        self.add_u32(zone)
    }

    /// Adds raw bytes to the datagram via an unsigned 8-bit integer vector.
    ///
    /// NOTE: not to be confused with add_blob(), which adds a dclass blob to the datagram.
    ///
    pub fn add_data(&mut self, mut v: Vec<u8>) -> Result<(), DatagramError> {
        if v.len() > globals::DG_SIZE_MAX.into() {
            // check input to avoid panic at .try_into() below
            return Err(DatagramError::DatagramOverflow(
                "Given bytes will overflow datagram.",
            ));
        }
        self.check_add_length(v.len().try_into().unwrap())?;
        self.buffer.append(&mut v);
        self.index += v.len();
        Ok(())
    }

    /// Adds a dclass string value to the end of the datagram.
    /// A 16-bit length tag prefix with the string's size in bytes is added.
    pub fn add_string(&mut self, v: &str) -> Result<(), DatagramError> {
        if v.len() > globals::DG_SIZE_MAX.into() {
            // The string is too big to be described with a 16-bit length tag.
            return Err(DatagramError::DatagramOverflow(
                "Given string will overflow datagram.",
            ));
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
    pub fn add_blob(&mut self, mut v: Vec<u8>) -> Result<(), DatagramError> {
        // add blob size in bytes
        self.add_size(v.len().try_into().unwrap())?;
        // manually check add length before appending byte array
        self.check_add_length(v.len().try_into().unwrap())?;
        self.buffer.append(&mut v);
        self.index += v.len();
        Ok(())
    }

    /// Reserves an amount of bytes in the datagram buffer.
    pub fn add_buffer(&mut self, bytes: globals::DgSizeTag) -> Result<globals::DgSizeTag, DatagramError> {
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
    ) -> Result<(), DatagramError> {
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
    pub fn add_control_header(&mut self, msg_type: globals::MsgType) -> Result<(), DatagramError> {
        self.add_u8(1)?;
        self.add_channel(globals::CONTROL_CHANNEL)?;
        self.add_u16(msg_type)?;
        Ok(())
    }

    pub fn size(&mut self) -> globals::DgSizeTag {
        self.buffer.len().try_into().unwrap()
    }

    pub fn get_data(&self) -> Vec<u8> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::Protocol;

    #[test]
    fn add_boolean() {
        let mut dg: Datagram = Datagram::default();
        let mut res: Result<(), DatagramError> = dg.add_bool(true);

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
    fn add_integers_and_types() {
        // A bit repetitive, but we need coverage on all of these methods.
        let mut dg: Datagram = Datagram::default();
        let mut results: Vec<Result<(), DatagramError>> = vec![];

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
    fn add_datagram() {
        let mut dg: Datagram = Datagram::default();
        let mut dg_2: Datagram = Datagram::default();

        assert!(dg.add_channel(globals::CHANNEL_MAX).is_ok());
        assert!(dg_2.add_blob(vec![0, 125, u8::MAX]).is_ok());

        let addition = dg.clone() + dg_2;

        assert!(addition.is_ok());
        dg = addition.unwrap();

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
    fn message_headers() {
        let mut dg: Datagram = Datagram::default();
        let mut results: Vec<Result<(), DatagramError>> = vec![];

        results.push(dg.add_server_header(
            vec![globals::CHANNEL_MAX], // recipients
            0, // sender
            Protocol::MDAddChannel.into(), // msg type
        ));

        results.push(dg.add_control_header(Protocol::MDAddChannel.into()));

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
    fn overflow_test() {
        let mut dg: Datagram = Datagram::default();
        let res_1: Result<globals::DgSizeTag, DatagramError> = dg.add_buffer(globals::DG_SIZE_MAX);

        assert!(!res_1.is_err(), "Could not append 2^16 bytes to datagram buffer.");
        assert_eq!(res_1.unwrap(), 0, "add_buffer() didn't return start of reserve.");
        assert_eq!(
            dg.size() + 1,
            globals::DG_SIZE_MAX,
            "Datagram didn't add 2^16 bytes to the buffer."
        );

        let res_2: Result<(), DatagramError> = dg.add_u16(0);
        assert!(
            res_2.is_err(),
            "Datagram overflow occurred, but did not throw an error."
        );

        assert_eq!(
            res_2.unwrap_err(),
            DatagramError::DatagramOverflow("Tried to add data to the datagram past its maximum size!"),
            "Datagram overflow occurred, but failed to respond with DatagramOverflow err."
        );
    }
}
