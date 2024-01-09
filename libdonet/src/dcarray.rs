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

use crate::dcnumeric::DCNumericRange;
use crate::dctype::{DCTypeDefinition, DCTypeEnum};
use crate::hashgen::DCHashGenerator;

pub struct DCArrayType {
    parent: DCTypeDefinition,
    element_type: DCTypeDefinition,
    size: usize,
    range: DCNumericRange,
}

pub trait DCArrayTypeInterface {
    fn new(element_type: DCTypeEnum, range: Option<DCNumericRange>) -> DCArrayType;
    fn generate_hash(&self, hashgen: &mut DCHashGenerator);

    fn has_range(&self) -> bool;

    fn get_range(&self) -> DCNumericRange;
    fn get_element_type(&self) -> DCTypeDefinition;
    fn get_array_size(&self) -> usize;
}

impl DCArrayTypeInterface for DCArrayType {
    fn new(element_type: DCTypeEnum, range: Option<DCNumericRange>) -> DCArrayType {
        todo!();
    }
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        todo!();
    }
    fn get_array_size(&self) -> usize {
        self.size.clone()
    }
    fn get_element_type(&self) -> DCTypeDefinition {
        self.element_type.clone()
    }
    fn get_range(&self) -> DCNumericRange {
        self.range.clone()
    }
    fn has_range(&self) -> bool {
        todo!();
    }
}
