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

//! Data model representing a DC Struct element. [NEEDS WORK]

use crate::dcfile::DCFile;
use crate::dconfig::*;
use crate::hashgen::*;

#[derive(Debug, Clone)]
pub struct DCStruct<'dc> {
    dcfile: &'dc DCFile<'dc>,
}

impl<'dc> std::fmt::Display for DCStruct<'dc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "TODO")
    }
}

impl<'dc> DCFileConfigAccessor for DCStruct<'dc> {
    fn get_dc_config(&self) -> &DCFileConfig {
        self.dcfile.get_dc_config()
    }
}

impl<'dc> LegacyDCHash for DCStruct<'dc> {
    fn generate_hash(&self, _: &mut DCHashGenerator) {
        // TODO
    }
}

/// Contains intermediate DC struct element structure and logic
/// for semantic analysis as the DC struct is being built.
pub(crate) mod interim {
    #[derive(Debug)]
    pub struct DCStruct {}
}
