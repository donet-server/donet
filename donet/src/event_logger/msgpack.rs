// DONET SOFTWARE
// Copyright (c) 2024, Donet Authors.
//
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

use libdonet::datagram::byte_order;
use libdonet::datagram::datagram::DatagramIterator;

#[rustfmt::skip]
static JSON_ESCAPES: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    'b' as u8, 't' as u8, 'n' as u8, 'v' as u8, 'f' as u8, 'r' as u8,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

/// [`MsgPack`] utility for decoding maps and arrays.
///
/// [`MsgPack`]: https://msgpack.org
fn decode_container(mut out: &mut String, mut dgi: &mut DatagramIterator, len: u32, map: bool) {
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
            decode_to_json(&mut out, &mut dgi);
            out.push_str(": ");
        }

        decode_to_json(&mut out, &mut dgi);
    }

    if map {
        out.push('}');
    } else {
        out.push(']');
    }
}

fn decode_string(mut out: &mut String, mut dgi: &mut DatagramIterator, len: u32) {
    out.push('"');

    for i in 0..len {
        let output: u8 = dgi.read_u8();

        if output < 0x20 {
            if JSON_ESCAPES.contains(&(output as u8)) {
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

fn decode_ext(mut out: &mut String, mut dgi: &mut DatagramIterator, len: u32) {
    out.push_str(&format!("ext({}, ", dgi.read_u8()));

    decode_string(&mut out, &mut dgi, len);
    out.push(')');
}

/// Utility for decoding a [`MsgPack`] datagram into a JSON-format UTF-8 string.
///
/// [`MsgPack`]: https://msgpack.org
pub fn decode_to_json(mut out: &mut String, mut dgi: &mut DatagramIterator) {
    let marker: u8 = dgi.read_u8();

    if marker < 0x80 {
        // positive fixint
        out.push_str(&format!("{}", marker));
    } else if marker <= 0x8f {
        // fixmap
        decode_container(&mut out, &mut dgi, (marker - 0x80).into(), true);
    } else if marker <= 0x9f {
        // fixarray
        decode_container(&mut out, &mut dgi, (marker - 0x90).into(), false);
    } else if marker <= 0xbf {
        // fixstr
        decode_string(&mut out, &mut dgi, (marker - 0xa0).into());
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
        decode_string(&mut out, &mut dgi, len.into());
    } else if marker == 0xc5 {
        // bin16
        let len: u16 = dgi.read_u16();
        decode_string(&mut out, &mut dgi, byte_order::swap_be_16(len).into());
    } else if marker == 0xc6 {
        // bin32
        let len: u32 = dgi.read_u32();
        decode_string(&mut out, &mut dgi, byte_order::swap_be_32(len).into());
    } else if marker == 0xc7 {
        // ext8
        let len: u8 = dgi.read_u8();
        decode_ext(&mut out, &mut dgi, len.into());
    } else if marker == 0xc8 {
        // ext16
        let len: u16 = dgi.read_u16();
        decode_ext(&mut out, &mut dgi, byte_order::swap_be_16(len).into());
    } else if marker == 0xc9 {
        // ext32
        let len: u32 = dgi.read_u32();
        decode_ext(&mut out, &mut dgi, byte_order::swap_be_32(len).into());
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
    } else if marker == 0xd8 {
        // fixext
        decode_ext(&mut out, &mut dgi, 1 << (marker - 0xd4));
    } else if marker == 0xd9 {
        // str8
        let len: u8 = dgi.read_u8();
        decode_string(&mut out, &mut dgi, len.into());
    } else if marker == 0xda {
        // str16
        let len: u16 = dgi.read_u16();
        decode_string(&mut out, &mut dgi, byte_order::swap_be_16(len).into());
    } else if marker == 0xdb {
        // str32
        let len: u32 = dgi.read_u32();
        decode_string(&mut out, &mut dgi, byte_order::swap_be_32(len).into());
    } else if marker == 0xdc {
        // array16
        let len: u16 = dgi.read_u16();
        decode_container(&mut out, &mut dgi, byte_order::swap_be_16(len).into(), false);
    } else if marker == 0xdd {
        // array32
        let len: u32 = dgi.read_u32();
        decode_container(&mut out, &mut dgi, byte_order::swap_be_32(len).into(), false);
    } else if marker == 0xde {
        // map16
        let len: u16 = dgi.read_u16();
        decode_container(&mut out, &mut dgi, byte_order::swap_be_16(len).into(), true);
    } else if marker == 0xdf {
        // map32
        let len: u32 = dgi.read_u32();
        decode_container(&mut out, &mut dgi, byte_order::swap_be_32(len).into(), true);
    } else {
        // everything >= 0xe0 is a negative fixint.
        out.push_str(&format!("{}", marker as i8));
    }
}
