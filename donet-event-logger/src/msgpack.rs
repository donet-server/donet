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

use donet_core::datagram::byte_order;
use donet_core::datagram::iterator::DatagramIterator;

#[rustfmt::skip]
static JSON_ESCAPES: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    b'b', b't', b'n', b'v', b'f', b'r',
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

/// [`MsgPack`] utility for decoding maps and arrays.
///
/// [`MsgPack`]: https://msgpack.org
fn decode_container(out: &mut String, dgi: &mut DatagramIterator, len: u32, map: bool) {
    if map {
        out.push('{');
    } else {
        out.push('[');
    }

    for i in 0..len {
        if i != 0 {
            out.push_str(", ");
        }

        if map {
            decode_to_json(out, dgi);
            out.push_str(": ");
        }

        decode_to_json(out, dgi);
    }

    if map {
        out.push('}');
    } else {
        out.push(']');
    }
}

fn decode_string(out: &mut String, dgi: &mut DatagramIterator, len: u32) {
    out.push('"');

    for _i in 0..len {
        let output: u8 = dgi.read_u8();

        if output < 0x20 {
            if JSON_ESCAPES.contains(&output) {
                out.push_str(&format!("\\{}", JSON_ESCAPES[output as usize]));
                continue;
            }
        } else if (output as char) == '"' {
            out.push_str("\\\"");
            continue;
        } else if (output as char) == '\\' {
            out.push_str("\\\\");
            continue;
        } else if output < 0x7f {
            out.push(output as char);
            continue;
        }

        out.push_str(&format!("\\x{:0width$x}", output, width = 2)); // 2 hex / byte
    }
    out.push('"');
}

fn decode_ext(out: &mut String, dgi: &mut DatagramIterator, len: u32) {
    out.push_str(&format!("ext({}, ", dgi.read_u8()));

    decode_string(out, dgi, len);
    out.push(')');
}

/// Utility for decoding a [`MsgPack`] datagram into a JSON-format UTF-8 string.
///
/// [`MsgPack`]: https://msgpack.org
pub fn decode_to_json(out: &mut String, dgi: &mut DatagramIterator) {
    let marker: u8 = dgi.read_u8();

    if marker < 0x80 {
        // positive fixint
        out.push_str(&format!("{}", marker));
    } else if marker <= 0x8f {
        // fixmap
        decode_container(out, dgi, (marker - 0x80).into(), true);
    } else if marker <= 0x9f {
        // fixarray
        decode_container(out, dgi, (marker - 0x90).into(), false);
    } else if marker <= 0xbf {
        // fixstr
        decode_string(out, dgi, (marker - 0xa0).into());
    } else if marker == 0xc0 {
        // nil
        out.push_str("null");
    } else if marker == 0xc1 {
        // (never used)
        out.push_str("*INVALID*");
    } else if marker == 0xc2 {
        // false
        out.push_str("false");
    } else if marker == 0xc3 {
        // true
        out.push_str("true");
    } else if marker == 0xc4 {
        // bin8
        let len: u8 = dgi.read_u8();
        decode_string(out, dgi, len.into());
    } else if marker == 0xc5 {
        // bin16
        let len: u16 = dgi.read_u16();
        decode_string(out, dgi, byte_order::swap_be_16(len).into());
    } else if marker == 0xc6 {
        // bin32
        let len: u32 = dgi.read_u32();
        decode_string(out, dgi, byte_order::swap_be_32(len));
    } else if marker == 0xc7 {
        // ext8
        let len: u8 = dgi.read_u8();
        decode_ext(out, dgi, len.into());
    } else if marker == 0xc8 {
        // ext16
        let len: u16 = dgi.read_u16();
        decode_ext(out, dgi, byte_order::swap_be_16(len).into());
    } else if marker == 0xc9 {
        // ext32
        let len: u32 = dgi.read_u32();
        decode_ext(out, dgi, byte_order::swap_be_32(len));
    } else if marker == 0xca {
        // float32
        let data: u32 = dgi.read_u32();
        out.push_str(&format!("{}", byte_order::swap_be_32(data) as f32));
    } else if marker == 0xcb {
        // float64
        let data: u64 = dgi.read_u64();
        out.push_str(&format!("{}", byte_order::swap_be_64(data) as f64));
    } else if marker == 0xcc {
        // uint8
        out.push_str(&format!("{}", dgi.read_u8()));
    } else if marker == 0xcd {
        // uint16
        out.push_str(&format!("{}", byte_order::swap_be_16(dgi.read_u16())));
    } else if marker == 0xce {
        // uint32
        out.push_str(&format!("{}", byte_order::swap_be_32(dgi.read_u32())));
    } else if marker == 0xcf {
        // uint64
        out.push_str(&format!("{}", byte_order::swap_be_64(dgi.read_u64())));
    } else if marker == 0xd0 {
        // int8
        out.push_str(&format!("{}", dgi.read_i8()));
    } else if marker == 0xd1 {
        // int16
        out.push_str(&format!("{}", byte_order::swap_be_16(dgi.read_u16()) as i16));
    } else if marker == 0xd2 {
        // int32
        out.push_str(&format!("{}", byte_order::swap_be_32(dgi.read_u32()) as i32));
    } else if marker == 0xd3 {
        // int64
        out.push_str(&format!("{}", byte_order::swap_be_64(dgi.read_u64()) as i64));
    } else if marker <= 0xd8 {
        // fixext family
        decode_ext(out, dgi, 1 << (marker - 0xd4));
    } else if marker == 0xd9 {
        // str8
        let len: u8 = dgi.read_u8();
        decode_string(out, dgi, len.into());
    } else if marker == 0xda {
        // str16
        let len: u16 = dgi.read_u16();
        decode_string(out, dgi, byte_order::swap_be_16(len).into());
    } else if marker == 0xdb {
        // str32
        let len: u32 = dgi.read_u32();
        decode_string(out, dgi, byte_order::swap_be_32(len));
    } else if marker == 0xdc {
        // array16
        let len: u16 = dgi.read_u16();
        decode_container(out, dgi, byte_order::swap_be_16(len).into(), false);
    } else if marker == 0xdd {
        // array32
        let len: u32 = dgi.read_u32();
        decode_container(out, dgi, byte_order::swap_be_32(len), false);
    } else if marker == 0xde {
        // map16
        let len: u16 = dgi.read_u16();
        decode_container(out, dgi, byte_order::swap_be_16(len).into(), true);
    } else if marker == 0xdf {
        // map32
        let len: u32 = dgi.read_u32();
        decode_container(out, dgi, byte_order::swap_be_32(len), true);
    } else {
        // everything >= 0xe0 is a negative fixint.
        out.push_str(&format!("{}", marker as i8));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use donet_core::datagram::datagram::{Datagram, DatagramError};

    #[test]
    fn fixmap_fixstr_common_usage() -> Result<(), DatagramError> {
        let mut output: String = String::default();

        // Unit tests must not be dependent on other tests, so we cannot
        // use any internal utilities to build the test datagram. (e.g. LoggedEvent)
        //
        // This does not include libdonet utilities, as they are a separate crate.
        let mut dg: Datagram = Datagram::default();

        dg.add_data(vec![0x80 + 0x3, 0xa0 + 0x4])?; // fixmap (3), fixstr (4)
        dg.add_data("test".as_bytes().to_vec())?; // "test"
        dg.add_data(vec![0xc3, 0xa0 + 0x4])?; // true, fixstr (4)
        dg.add_data("test".as_bytes().to_vec())?; // "test"
        dg.add_data(vec![0x3])?; // positive fixint (3)
        dg.add_data(vec![0xa0 + 0x4])?; // fixstr (4)
        dg.add_data("test".as_bytes().to_vec())?; // "test"
        dg.add_data(vec![0xc0])?; // null

        decode_to_json(&mut output, &mut DatagramIterator::from(dg));

        assert_eq!(output.as_str(), "{\"test\": true, \"test\": 3, \"test\": null}");
        Ok(())
    }

    #[test]
    fn fixarray_container_decode() -> Result<(), DatagramError> {
        let mut output: String = String::default();
        let mut dg: Datagram = Datagram::default();

        dg.add_data(vec![0x90 + 0x3])?; // fixarray (3)
        dg.add_data(vec![0xc2])?; // false
        dg.add_data(vec![0xc1])?; // unused marker
        dg.add_data(vec![0xe0])?; // negative fixint (-32)

        decode_to_json(&mut output, &mut DatagramIterator::from(dg));

        assert_eq!(output.as_str(), "[false, *INVALID*, -32]");
        Ok(())
    }

    #[test]
    fn fixext_decode() -> Result<(), DatagramError> {
        let mut output: String = String::default();
        let mut dg: Datagram = Datagram::default();

        dg.add_data(vec![0xd4])?; // fixext 1
        dg.add_data(vec![0x1])?; // type
        dg.add_data(vec![0xff])?; // value

        decode_to_json(&mut output, &mut DatagramIterator::from(dg));

        assert_eq!(output.as_str(), "ext(1, \"\\xff\")");
        Ok(())
    }

    #[test]
    fn msgpack_floating_types() -> Result<(), DatagramError> {
        let mut output: String = String::default();
        let mut dg: Datagram = Datagram::default();

        dg.add_data(vec![0x90 + 0x2])?; // fixarray (2)
        dg.add_data(vec![0xca])?; // float32
        dg.add_f32(f32::MAX)?; // value
        dg.add_data(vec![0xcb])?; // float64
        dg.add_f64(f64::MAX)?; // value

        decode_to_json(&mut output, &mut DatagramIterator::from(dg));

        assert_eq!(output.as_str(), "[4294967300, 18446744073709552000]");
        Ok(())
    }
}
