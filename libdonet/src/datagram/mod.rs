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

//! This module provides [`datagram::Datagram`] and [`datagram::DatagramIterator`].
//!
//! Solutions provided by this module:
//!
//! - Constructing datagrams with appropriate headers and payloads.
//! - Iterating through and extracting information from received datagrams.
//! - Converting endianness of datagram bytes to native byte order.
//! - Datagram-level error handling.

pub mod byte_order;
pub mod datagram;
