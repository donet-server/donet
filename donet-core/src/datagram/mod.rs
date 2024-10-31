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

//! This module provides [`datagram::Datagram`] and [`iterator::DatagramIterator`].
//!
//! Solutions provided by this module:
//!
//! - Constructing datagrams with appropriate headers and payloads.
//! - Iterating through and extracting information from received datagrams.
//! - Converting endianness of datagram bytes to native byte order.
//! - Datagram-level error handling.

pub mod byte_order;
pub mod datagram;
pub mod iterator;
