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

use cfg_if::cfg_if;
use std::mem;
use std::result::Result;
use strum_macros::EnumIter;

// ---------- Type Definitions --------- //

pub type MsgType = u16;
pub type DgSizeTag = u16;
pub type Channel = u64;
pub type DoId = u32;
pub type Zone = u32;
pub type DClassId = u16;
pub type FieldId = u16;
pub type DCFileHash = u32; // 32-bit hash

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
        pub static DC_MULTIPLE_INHERITANCE: bool = true;
        pub static DC_VIRTUAL_INHERITANCE: bool = true;
        pub static DC_SORT_INHERITANCE_BY_FILE: bool = false;
        pub static MAX_PRIME_NUMBERS: u16 = 10000;
    }
}

// ---------- Datagram Feature ---------- //

cfg_if! {
    if #[cfg(feature = "datagram")] {

        // All possible errors that can be returned by
        // the Datagram and DatagramIterator implementations.
        #[derive(Debug, PartialEq)]
        pub enum DgError {
            DatagramOverflow,
            DatagramIteratorEOF,
            //FieldConstraintViolation,
        }

        pub type DgResult = Result<(), DgError>;
        pub type DgBufferResult = Result<DgSizeTag, DgError>;
    }
}

// ---------- Network Protocol ---------- //

/// Utility for converting protocol enumerator to u16 (MsgType)
pub fn msg_type(proto_enum: Protocol) -> MsgType {
    proto_enum as MsgType
}

#[repr(u16)] // 16-bit alignment
#[derive(Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum Protocol {
    ClientHello = 1,
    ClientHelloResp = 2,
    ClientDisconnect = 3,
    ClientEject = 4,
    ClientHeartbeat = 5,

    ClientObjectSetField = 120,
    ClientObjectSetFields = 121,
    ClientObjectLeaving = 132,
    ClientObjectLeavingOwner = 161,
    ClientEnterObjectRequired = 142,
    ClientEnterObjectRequiredOther = 143,
    ClientEnterObjectRequiredOwner = 172,
    ClientEnterObjectRequiredOwnerOther = 173,

    ClientDoneInterestResp = 204,

    ClientAddInterest = 200,
    ClientAddInterestMultiple = 201,
    ClientRemoveInterest = 203,
    ClientObjectLocation = 140,

    // Client Agent
    CASetState = 1000,
    CASetClientID = 1001,
    CASendDatagram = 1002,
    CAEject = 1004,
    CADrop = 1005,
    CAGetNetworkAddress = 1006,
    CAGetNetworkAddressResp = 1007,
    CADeclareObject = 1010,
    CAUndeclareObject = 1011,
    CAAddSessionObject = 1012,
    CARemoveSessionObject = 1013,
    CASetFieldsSendable = 1014,
    CAOpenChannel = 1100,
    CACloseChannel = 1101,
    CAAddPostRemove = 1110,
    CAClearPostRemoves = 1111,
    CAAddInterest = 1200,
    CAAddInterestMultiple = 1201,
    CARemoveInterest = 1203,

    // State Server
    SSCreateObjectWithRequired = 2000,
    SSCreateObjectWithRequiredOther = 2001,
    SSDeleteAIObjects = 2009,
    SSObjectGetField = 2010,
    SSObjectGetFieldResp = 2011,
    SSObjectGetFields = 2012,
    SSObjectGetFieldsResp = 2013,
    SSObjectGetAll = 2014,
    SSObjectGetAllResp = 2015,
    SSObjectSetField = 2020,
    SSObjectSetFields = 2021,
    SSObjectDeleteFieldRAM = 2030,
    SSObjectDeleteFieldsRAM = 2031,
    SSObjectDeleteRAM = 2032,
    SSObjectSetLocation = 2040,
    SSObjectChangingLocation = 2041,
    SSObjectEnterLocationWithRequired = 2042,
    SSObjectEnterLocationWithRequiredOther = 2043,
    SSObjectGetLocation = 2044,
    SSObjectGetLocationResp = 2045,
    SSObjectSetAI = 2050,
    SSObjectChangingAI = 2051,
    SSObjectEnterAIWithRequired = 2052,
    SSObjectEnterAIWithRequiredOther = 2053,
    SSObjectGetAI = 2054,
    SSObjectGetAIResp = 2055,
    SSObjectSetOwner = 2060,
    SSObjectChangingOwner = 2061,
    SSObjectEnterOwnerWithRequired = 2062,
    SSObjectEnterOwnerWithRequiredOther = 2063,
    SSObjectGetOwner = 2064,
    SSObjectGetOwnerResp = 2065,
    SSObjectGetZoneObjects = 2100,
    SSObjectGetZonesObjects = 2102,
    SSObjectGetChildren = 2104,
    SSObjectGetZoneCount = 2110,
    SSObjectGetZoneCountResp = 2111,
    SSObjectGetZonesCount = 2112,
    SSObjectGetZonesCountResp = 2113,
    SSObjectGetChildCount = 2114,
    SSObjectGetChildCountResp = 2115,
    SSObjectDeleteZone = 2120,
    SSObjectDeleteZones = 2122,
    SSObjectDeleteChildren = 2124,

    // Database State Server
    DBSSObjectActivateWithDefaults = 2200,
    DBSSObjectActivateWithDefaultsOther = 2201,
    DBSSObjectGetActivated = 2207,
    DBSSObjectGetActivatedResp = 2208,
    DBSSObjectDeleteFieldDisk = 2230,
    DBSSObjectDeleteFieldsDisk = 2231,
    DBSSObjectDeleteDisk = 2232,

    // Database Server
    DBCreateObject = 3000,
    DBCreateObjectResp = 3001,
    DBObjectGetField = 3010,
    DBObjectGetFieldResp = 3011,
    DBObjectGetFields = 3012,
    DBObjectGetFieldsResp = 3013,
    DBObjectGetAll = 3014,
    DBObjectGetAllResp = 3015,
    DBObjectSetField = 3020,
    DBObjectSetFields = 3021,
    DBObjectSetFieldIfEquals = 3022,
    DBObjectSetFieldIfEqualsResp = 3023,
    DBObjectSetFieldsIfEquals = 3024,
    DBObjectSetFieldsIfEqualsResp = 3025,
    DBObjectSetFieldIfEmpty = 3026,
    DBObjectSetFieldIfEmptyResp = 3027,
    DBObjectDeleteField = 3030,
    DBObjectDeleteFields = 3031,
    DBObjectDelete = 3032,

    // Message Director (Control)
    MDAddChannel = 9000,
    MDRemoveChannel = 9001,
    MDAddRange = 9002,
    MDRemoveRange = 9003,
    MDAddPostRemove = 9010,
    MDClearPostRemoves = 9011,
}

#[cfg(test)]
mod unit_testing {
    use super::{msg_type, Protocol};

    #[test]
    fn test_protocol_to_u16_util() {
        assert_eq!(msg_type(Protocol::MDRemoveChannel), 9001);
        assert_eq!(msg_type(Protocol::CAAddInterest), 1200);
        assert_eq!(msg_type(Protocol::SSDeleteAIObjects), 2009);
    }
}
