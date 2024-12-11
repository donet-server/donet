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

//! Utils for swapping little-endian bytes to the compiling
//! processor's native endianness (byte order).

/// Swaps 2 bytes in little endian byte order to big endian.
/// Returns the input if the processor is little endian.
#[cfg(target_endian = "big")]
pub fn swap_le_16(v: u16) -> u16 {
    (v & 0x00ff) << 8 | (v & 0xff00) >> 8
}

/// Swaps 4 bytes in little endian byte order to big endian.
/// Returns the input if the processor is little endian.
#[rustfmt::skip]
#[cfg(target_endian = "big")]
pub fn swap_le_32(v: u32) -> u32 {
    (v & 0x000000ff) << 24
    | (v & 0x0000ff00) << 8
    | (v & 0x00ff0000) >> 8
    | (v & 0xff000000) >> 24
}

/// Swaps 8 bytes in little endian byte order to big endian.
/// Returns the input if the processor is little endian.
#[cfg(target_endian = "big")]
#[rustdoc::doc(hidden)]
pub fn swap_le_64(v: u64) -> u64 {
    (v & 0x00000000000000ff) << 56
        | (v & 0x000000000000ff00) << 40
        | (v & 0x0000000000ff0000) << 24
        | (v & 0x00000000ff000000) << 8
        | (v & 0x000000ff00000000) >> 8
        | (v & 0x0000ff0000000000) >> 24
        | (v & 0x00ff000000000000) >> 40
        | (v & 0xff00000000000000) >> 56
}

/// Swaps 2 bytes in little endian byte order to big endian.
/// Returns the input if the processor is little endian.
#[cfg(target_endian = "little")]
pub fn swap_le_16(v: u16) -> u16 {
    v // no need to swap bytes
}

/// Swaps 4 bytes in little endian byte order to big endian.
/// Returns the input if the processor is little endian.
#[cfg(target_endian = "little")]
pub fn swap_le_32(v: u32) -> u32 {
    v
}

/// Swaps 8 bytes in little endian byte order to big endian.
/// Returns the input if the processor is little endian.
#[cfg(target_endian = "little")]
pub fn swap_le_64(v: u64) -> u64 {
    v
}

/// Swaps 2 bytes in big endian byte order to little endian.
/// Returns the input if the processor is big endian.
#[cfg(target_endian = "little")]
pub fn swap_be_16(v: u16) -> u16 {
    (v & 0x00ff) << 8 | (v & 0xff00) >> 8
}

/// Swaps 4 bytes in big endian byte order to little endian.
/// Returns the input if the processor is big endian.
#[rustfmt::skip]
#[cfg(target_endian = "little")]
pub fn swap_be_32(v: u32) -> u32 {
    (v & 0x000000ff) << 24
    | (v & 0x0000ff00) << 8
    | (v & 0x00ff0000) >> 8
    | (v & 0xff000000) >> 24
}

/// Swaps 8 bytes in big endian byte order to little endian.
/// Returns the input if the processor is big endian.
#[cfg(target_endian = "little")]
pub fn swap_be_64(v: u64) -> u64 {
    (v & 0x00000000000000ff) << 56
        | (v & 0x000000000000ff00) << 40
        | (v & 0x0000000000ff0000) << 24
        | (v & 0x00000000ff000000) << 8
        | (v & 0x000000ff00000000) >> 8
        | (v & 0x0000ff0000000000) >> 24
        | (v & 0x00ff000000000000) >> 40
        | (v & 0xff00000000000000) >> 56
}

/// Swaps 2 bytes in big endian byte order to little endian.
/// Returns the input if the processor is big endian.
#[cfg(target_endian = "big")]
pub fn swap_be_16(v: u16) -> u16 {
    v // no need to swap bytes
}

/// Swaps 4 bytes in big endian byte order to little endian.
/// Returns the input if the processor is big endian.
#[cfg(target_endian = "big")]
pub fn swap_be_32(v: u32) -> u32 {
    v
}

/// Swaps 8 bytes in big endian byte order to little endian.
/// Returns the input if the processor is big endian.
#[cfg(target_endian = "big")]
pub fn swap_be_64(v: u64) -> u64 {
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    // Little-endian swap tests

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

    // Big-endian swap tests

    #[test]
    #[cfg(target_endian = "little")]
    fn endianness_swap_be_16() -> () {
        let res: u16 = swap_be_16(1000 as u16);
        assert_eq!(res, 59395);
    }

    #[test]
    #[cfg(target_endian = "big")]
    fn endianness_swap_be_16() -> () {
        let res: u16 = swap_be_16(1000 as u16);
        assert_eq!(res, 1000);
    }

    #[test]
    #[cfg(target_endian = "little")]
    fn endianness_swap_be_32() -> () {
        let res: u32 = swap_be_32(100000000 as u32);
        assert_eq!(res, 14808325);
    }

    #[test]
    #[cfg(target_endian = "big")]
    fn endianness_swap_be_32() -> () {
        let res: u32 = swap_be_32(100000000 as u32);
        assert_eq!(res, 100000000);
    }

    #[test]
    #[cfg(target_endian = "little")]
    fn endianness_swap_be_64() -> () {
        let res: u64 = swap_be_64(100000000000000000 as u64);
        assert_eq!(res, 152134054404865);
    }

    #[test]
    #[cfg(target_endian = "big")]
    fn endianness_swap_be_64() -> () {
        let res: u64 = swap_be_64(100000000000000000 as u64);
        assert_eq!(res, 100000000000000000);
    }
}
