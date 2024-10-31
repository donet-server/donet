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

//! Global configuration variables for the DC parser pipeline.

/// Stored in the [`crate::dcfile::DCFile`] structure.
///
/// Configuration variables to how the DC parser pipeline
/// handles the semantics of the DC file(s) being read.
#[derive(Debug, Clone)]
pub struct DCFileConfig {
    /// Set this true to support multiple inheritance in the dc
    /// file. If this is false, the old way, multiple inheritance
    /// is not supported, but field numbers will be numbered
    /// sequentially, which may be required to support old code
    /// that assumed this.
    pub dc_multiple_inheritance: bool,
    /// This is a temporary hack. This should be true if you are
    /// using version 1.42 of the otp_server.exe binary, which
    /// sorted inherited fields based on the order of the classes
    /// within the DC file, rather than based on the order in
    /// which the references are made within the class.
    pub dc_sort_inheritance_by_file: bool,
    /// Set this true to support proper virtual inheritance in
    /// the dc file, so that diamond-of-death type constructs can
    /// be used. This also enables shadowing (overloading) of
    /// inherited method names from a base class.
    pub dc_virtual_inheritance: bool,
}

/// Creates the config struct with Panda's defaults.
impl Default for DCFileConfig {
    fn default() -> Self {
        Self {
            dc_multiple_inheritance: true,
            dc_sort_inheritance_by_file: true,
            dc_virtual_inheritance: true,
        }
    }
}

impl std::fmt::Display for DCFileConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "/*")?;
        writeln!(f, "DC_MULTIPLE_INHERITANCE = {}", self.dc_multiple_inheritance)?;
        writeln!(
            f,
            "DC_SORT_INHERITANCE_BY_FILE = {}",
            self.dc_sort_inheritance_by_file,
        )?;
        writeln!(f, "DC_VIRTUAL_INHERITANCE = {}", self.dc_virtual_inheritance)?;
        writeln!(f, "*/\n")
    }
}

/// All DC element structures with a pointer to the
/// DC file configuration struct should implement this.
pub trait DCFileConfigAccessor {
    fn get_dc_config(&self) -> &DCFileConfig;
}
