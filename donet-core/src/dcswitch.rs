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

//! Data model that represents a DC switch statement.

use crate::dcfield::DCField;
use crate::hashgen::*;
use std::collections::HashMap;

/// Represents a case in a DC switch declaration.
#[derive(Debug)]
pub struct SwitchCase<'dc> {
    switch: &'dc DCSwitch<'dc>,
    /// Note that in the legacy DC language, switch cases
    /// always assume to break, no matter if a break
    /// statement was parsed at syntax analysis. This
    /// legacy behavior is not followed by Donet.
    breaks: bool,
    /// Empty byte array signifies default case.
    value: Vec<u8>,
    fields: Vec<DCField<'dc>>,
}

impl std::fmt::Display for SwitchCase<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_default() {
            writeln!(f, "default:")?;
        } else {
            write!(f, "case ")?;
            self.switch.key.format_packed_data(f, &self.value, false)?;
            writeln!(f, ":")?;
        }

        for field in &self.fields {
            f.write_str(&field.to_string())?;
        }
        if self.breaks {
            writeln!(f, "break;")?;
        }
        Ok(())
    }
}

impl LegacyDCHash for SwitchCase<'_> {
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        if !self.is_default() {
            hashgen.add_blob(self.value.clone());
        }

        hashgen.add_int(self.get_num_fields() as i32);

        for field in &self.fields {
            field.generate_hash(hashgen);
        }
    }
}

impl<'dc> SwitchCase<'dc> {
    /// Returns true if this case is a default case.
    pub fn is_default(&self) -> bool {
        self.value.is_empty()
    }

    /// Returns the number of fields in the case.
    pub fn get_num_fields(&self) -> usize {
        self.fields.len()
    }

    pub fn get_field(&self, index: usize) -> Option<&'dc DCField> {
        self.fields.get(index)
    }

    pub fn get_field_by_name(&self, _name: String) -> Option<&'dc DCField> {
        todo!()
    }
}

/// Represents a DC Switch statement, which can appear inside
/// a dclass declaration and represents two or more alternative
/// unpacking schemes based on the first field read.
#[derive(Debug)]
pub struct DCSwitch<'dc> {
    name: Option<String>,
    key: DCField<'dc>,
    cases: Vec<SwitchCase<'dc>>,
    default_case: Option<SwitchCase<'dc>>,
    case_fields: Vec<&'dc DCField<'dc>>,
    cases_by_value: HashMap<Vec<u8>, usize>,
}

impl From<interim::DCSwitch> for DCSwitch<'_> {
    fn from(value: interim::DCSwitch) -> Self {
        Self {
            name: value.name,
            key: todo!(),
            cases: vec![],
            default_case: None,
            case_fields: vec![],
            cases_by_value: HashMap::default(),
        }
    }
}

impl std::fmt::Display for DCSwitch<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "switch")?;

        if let Some(name) = &self.name {
            write!(f, " {}", name)?;
        }
        write!(f, " (")?;
        f.write_str(&self.key.to_string())?;

        writeln!(f, ") {{")?;

        for case in &self.cases {
            f.write_str(&case.to_string())?;
        }
        writeln!(f, "}};")
    }
}

impl LegacyDCHash for DCSwitch<'_> {
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        if let Some(name) = self.get_name() {
            hashgen.add_string(name)
        }

        self.key.generate_hash(hashgen);

        hashgen.add_int(self.get_num_cases() as i32);

        for case in &self.cases {
            case.generate_hash(hashgen);
        }

        if let Some(default) = &self.default_case {
            default.generate_hash(hashgen);
        }
    }
}

impl<'dc> DCSwitch<'dc> {
    /// Returns the optional identifier for this switch.
    #[inline(always)]
    pub fn get_name(&self) -> Option<String> {
        self.name.clone()
    }

    /// Returns the key parameter on which the switch is based.
    ///
    /// The value of this parameter in the record determines which
    /// one of the several cases within the switch will be used.
    #[inline(always)]
    pub fn get_key_parameter(&self) -> &'dc DCField {
        &self.key
    }

    /// Returns the number of different cases within the switch.
    ///
    /// The legal values for case_index range from 0 to n_cases - 1.
    #[inline(always)]
    pub fn get_num_cases(&self) -> usize {
        self.cases.len()
    }

    /// Returns case reference from given index wrapped in an Option.
    pub fn get_case(&self, index: usize) -> Option<&'dc SwitchCase> {
        self.cases.get(index)
    }

    /// Returns default case reference wrapped in an Option.
    ///
    /// A default case is optional, so `None` can be returned.
    pub fn get_default_case(&self) -> Option<&'dc SwitchCase> {
        self.default_case.as_ref()
    }

    /// Returns the index of the case with the given packed value.
    ///
    /// `None` is returned if no case with that value is found.
    pub fn get_case_index_by_value(&self, value: Vec<u8>) -> Option<usize> {
        self.cases_by_value.get(&value).copied()
    }

    // TODO
    pub fn apply_switch(&self, _value: Vec<u8>, _length: usize) {}
}

/// Contains intermediate DC Switch structure and logic
/// for semantic analysis as the DC Switch is being built.
pub(crate) mod interim {
    #[derive(Debug)]
    pub struct SwitchCase {
        pub breaks: bool,
        pub value: Vec<u8>,
    }

    impl SwitchCase {
        fn add_case_field(&mut self) {}
    }

    #[derive(Debug)]
    pub struct DCSwitch {
        pub name: Option<String>,
        pub key_parameter: u8, // TODO
    }

    impl DCSwitch {
        fn add_case(&mut self) {}
    }
}
