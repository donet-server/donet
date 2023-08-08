// DONET SOFTWARE
// Copyright (c) 2023, DoNet Authors.

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

#[path = "types.rs"]
mod type_aliases;

#[allow(dead_code)]
pub mod results {
    use super::type_aliases::type_aliases as types;
    use std::error::Error;
    use std::result::Result; // not to be confused with std::io::Result

    // All possible errors that can be returned by
    // the Datagram and DatagramIterator implementations.
    #[derive(Debug, PartialEq)]
    pub enum DgError {
        DatagramOverflow,
        DatagramIteratorEOF,
        //FieldConstraintViolation,
    }

    pub type DgResult = Result<(), DgError>;
    pub type DgBufferResult = Result<types::DgSize, DgError>;

    // MySQL Result (mysql crate API response)
    pub type SqlResult = Result<(), Box<dyn Error>>;
}
