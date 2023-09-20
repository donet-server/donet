// DONET SOFTWARE
// Copyright (c) 2023, Donet Authors.
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

// Detect system endianness (byte order)
#[cfg(target_endian = "big")]
pub fn swap_le_16(v: u16) -> u16 {
    return (v & 0x00ff) << 8 | (v & 0xff00) >> 8;
}

#[rustfmt::skip]
#[cfg(target_endian = "big")]
pub fn swap_le_32(v: u32) -> u32 {
    return (v & 0x000000ff) << 24
        | (v & 0x0000ff00) << 8
        | (v & 0x00ff0000) >> 8
        | (v & 0xff000000) >> 24;
}

#[cfg(target_endian = "big")]
pub fn swap_le_64(v: u64) -> u64 {
    return (v & 0x00000000000000ff) << 56
        | (v & 0x000000000000ff00) << 40
        | (v & 0x0000000000ff0000) << 24
        | (v & 0x00000000ff000000) << 8
        | (v & 0x000000ff00000000) >> 8
        | (v & 0x0000ff0000000000) >> 24
        | (v & 0x00ff000000000000) >> 40
        | (v & 0xff00000000000000) >> 56;
}

#[cfg(target_endian = "little")]
pub fn swap_le_16(v: u16) -> u16 {
    v // no need to swap bytes
}

#[cfg(target_endian = "little")]
pub fn swap_le_32(v: u32) -> u32 {
    v
}

#[cfg(target_endian = "little")]
pub fn swap_le_64(v: u64) -> u64 {
    v
}

#[cfg(test)]
mod unit_testing {
    use super::*;

    #[test]
    #[cfg(target_endian = "big")]
    fn endianness_swap_le_16() -> () {
        let res: u16 = swap_le_16(1000 as u16);
        assert_eq!(res, 59395);
    }

    #[test]
    #[cfg(target_endian = "little")]
    fn endianness_swap_le_16() -> () {
        let res: u16 = swap_le_16(1000 as u16);
        assert_eq!(res, 1000);
    }

    #[test]
    #[cfg(target_endian = "big")]
    fn endianness_swap_le_32() -> () {
        let res: u32 = swap_le_32(100000000 as u32);
        assert_eq!(res, 14808325);
    }

    #[test]
    #[cfg(target_endian = "little")]
    fn endianness_swap_le_32() -> () {
        let res: u32 = swap_le_32(100000000 as u32);
        assert_eq!(res, 100000000);
    }

    #[test]
    #[cfg(target_endian = "big")]
    fn endianness_swap_le_64() -> () {
        let res: u64 = swap_le_64(100000000000000000 as u64);
        assert_eq!(res, 152134054404865);
    }

    #[test]
    #[cfg(target_endian = "little")]
    fn endianness_swap_le_64() -> () {
        let res: u64 = swap_le_64(100000000000000000 as u64);
        assert_eq!(res, 100000000000000000);
    }
}
