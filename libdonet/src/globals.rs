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

//! Includes definitions of type aliases for Donet concepts,
//! and the full definition of the network protocol message types.

use super::protocol::*;
use cfg_if::cfg_if;
use std::mem;

// ---------- Type Definitions --------- //

pub type MsgType = u16;
pub type DgSizeTag = u16;
pub type Channel = u64;
pub type DoId = u32;
pub type Zone = u32;
pub type DClassId = u16;
pub type FieldId = u16;
pub type DCFileHash = u32; // 32-bit hash

/// Impl converting protocol enumerator to u16 (MsgType)
impl From<Protocol> for MsgType {
    fn from(value: Protocol) -> Self {
        value as MsgType
    }
}

// ---------- Type Limits ---------- //

pub const DG_SIZE_MAX: DgSizeTag = u16::MAX;
pub const CHANNEL_MAX: Channel = u64::MAX;
pub const DOID_MAX: DoId = u32::MAX;
pub const ZONE_MAX: Zone = u32::MAX;
pub const ZONE_BITS: usize = 8 * mem::size_of::<Zone>();

// ---------- Constants ---------- //

pub const INVALID_DOID: DoId = 0;
pub const INVALID_CHANNEL: Channel = 0;
pub const CONTROL_CHANNEL: Channel = 1;
pub const BCHAN_CLIENTS: Channel = 10;
pub const BCHAN_STATESERVERS: Channel = 12;
pub const BCHAN_DBSERVERS: Channel = 13;

// ---------- DC File Feature ---------- //

cfg_if! {
    if #[cfg(feature = "dcfile")] {
        // DC File Constants
        pub static HISTORICAL_DC_KEYWORDS: &[&str] = &[
            "ram", "required", "db", "airecv", "ownrecv",
            "clrecv", "broadcast", "ownsend", "clsend",
        ];
        pub static DC_VIEW_SUFFIXES: &[&str] = &["AI", "OV", "UD"];
        pub static MAX_PRIME_NUMBERS: u16 = 10000;
    }
}

#[cfg(test)]
mod unit_testing {
    use super::*;

    #[test]
    fn msgtype_from_impl() {
        assert_eq!(MsgType::from(Protocol::MDRemoveChannel), 9001);
        assert_eq!(MsgType::from(Protocol::CAAddInterest), 1200);
        assert_eq!(MsgType::from(Protocol::SSDeleteAIObjects), 2009);
    }
}
