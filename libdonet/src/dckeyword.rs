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

//! Representation of arbitrary and historical
//! keywords as defined in the DC file.

use crate::hashgen::*;
use multimap::MultiMap;
use std::rc::Rc;

/// This is a flag bitmask for historical keywords.
/// Panda uses a C/C++ 'int' for this, which is stored
/// as 4 bytes in modern 32-bit and 64-bit C/C++ compilers.
pub type HistoricalFlag = i32;

#[derive(Debug, PartialEq, Eq)]
pub struct DCKeyword {
    name: String,
    // This flag is only kept for historical reasons, so we can
    // preserve the DC file's hash code if no new flags are in use.
    historical_flag: HistoricalFlag,
}

impl std::fmt::Display for DCKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "keyword ")?;
        f.write_str(&self.name)?;
        writeln!(f, ";")
    }
}

impl DCKeyword {
    pub fn new(name: String, historical_flag: Option<HistoricalFlag>) -> Self {
        if let Some(h_flag) = historical_flag {
            Self {
                name,
                historical_flag: h_flag,
            }
        } else {
            Self {
                name,
                historical_flag: !0, // bitwise complement
            }
        }
    }

    /// Accumulates the properties of this DC element into the file hash.
    pub fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        hashgen.add_string(self.name.clone());
    }

    #[inline]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    #[inline]
    pub fn get_historical_flag(&self) -> HistoricalFlag {
        self.historical_flag
    }

    /// Sets the historical flag bitmask to the bitwise complement of 0
    /// (!0 in Rust, or ~0 in C/C++), as if the keyword were not one
    /// of the historically defined keywords.
    pub fn clear_historical_flag(&mut self) {
        self.historical_flag = !0;
    }
}

/// A map of key/value pairs mapping keyword names to DCKeyword struct pointers.
pub type KeywordName2Keyword = MultiMap<String, Rc<DCKeyword>>;

/// Represents the two types of inputs that `DCKeywordList.has_keyword`
/// accepts for looking up a Keyword. In Panda and Astron, the
/// `has_keyword` method is overloaded instead.
pub enum IdentifyKeyword {
    ByStruct(DCKeyword),
    ByName(String),
}

#[derive(Debug)]
pub struct DCKeywordList {
    keywords: Vec<Rc<DCKeyword>>,
    kw_name_2_keyword: KeywordName2Keyword,
    flags: HistoricalFlag,
}

impl Default for DCKeywordList {
    fn default() -> Self {
        Self {
            keywords: vec![],
            kw_name_2_keyword: MultiMap::new(),
            // Panda initializes its keyword list class with the flags bitmask
            // set to 0 as a regular int (signed). But, it still confuses me why
            // since a clear bitmask (no historical kw flags) is the bitwise complement of 0.
            flags: 0_i32,
        }
    }
}

impl std::cmp::PartialEq for DCKeywordList {
    fn eq(&self, other: &Self) -> bool {
        let target_kw_map: KeywordName2Keyword = other._get_keywords_by_name_map();

        // If our maps are different sizes, they are already not the same.
        if self.kw_name_2_keyword.len() != target_kw_map.len() {
            return false;
        }

        // Since MultiMap does not implement the Eq trait,
        // we have to iterate through both maps and compare.
        for key in self.kw_name_2_keyword.keys() {
            if !target_kw_map.contains_key(key) {
                return false;
            }
        }
        true // no differences found
    }
}

impl DCHash for DCKeywordList {
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        if self.flags != !0 {
            // All of the flags are historical flags only, so add just the flags
            // bitmask to keep the hash code the same as it has historically been.
            hashgen.add_int(self.flags);
        } else {
            hashgen.add_int(self.keywords.len().try_into().unwrap());

            for kw_ptr in &self.keywords {
                kw_ptr.generate_hash(hashgen);
            }
        }
    }
}

impl DCKeywordList {
    pub fn add_keyword(&mut self, keyword: DCKeyword) -> Result<(), ()> {
        let kw_name: String = keyword.name.clone(); // avoid moving 'name'

        if self.kw_name_2_keyword.get(&kw_name).is_some() {
            return Err(()); // keyword is already in our list!
        }

        // Mixes the bitmask of this keyword into our KW list flags bitmask.
        self.flags |= keyword.get_historical_flag();

        self.keywords.push(Rc::new(keyword));
        self.kw_name_2_keyword
            .insert(kw_name, self.keywords.last().unwrap().clone());
        Ok(())
    }

    pub fn get_num_keywords(&self) -> usize {
        self.keywords.len()
    }

    pub fn has_keyword(&self, kw: IdentifyKeyword) -> bool {
        match kw {
            IdentifyKeyword::ByName(kw_id) => self.get_keyword_by_name(kw_id).is_some(),
            IdentifyKeyword::ByStruct(kw_obj) => {
                for kw_ptr in &self.keywords {
                    let keyword: Rc<DCKeyword> = Rc::clone(kw_ptr);

                    if *keyword == kw_obj {
                        return true;
                    }
                }
                false // no match found :(
            }
        }
    }

    pub fn get_keyword(&self, index: usize) -> Option<Rc<DCKeyword>> {
        self.keywords.get(index).map(Rc::clone)
    }

    pub fn get_keyword_by_name(&self, name: String) -> Option<Rc<DCKeyword>> {
        self.kw_name_2_keyword.get(&name).map(Rc::clone)
    }

    /// Returns a clone of this object's keyword array.
    pub fn _get_keyword_list(&self) -> Vec<Rc<DCKeyword>> {
        self.keywords.clone()
    }

    /// Returns a clone of this object's keyword name map.
    pub fn _get_keywords_by_name_map(&self) -> KeywordName2Keyword {
        self.kw_name_2_keyword.clone()
    }

    /// Overwrites the DCKeywords of this list with the target's DCKeywords.
    pub fn copy_keywords(&mut self, target: &DCKeywordList) {
        let target_kw_array: Vec<Rc<DCKeyword>> = target._get_keyword_list();
        let target_kw_map: KeywordName2Keyword = target._get_keywords_by_name_map();

        self.keywords = target_kw_array; // old vec will be dropped from memory
        self.kw_name_2_keyword = target_kw_map;
    }

    /// Clears the DCKeywords array, keyword name map, and
    /// historical flags bitmask from this [`DCKeywordList`] struct.
    pub fn clear_keywords(&mut self) {
        self.keywords.clear();
        self.kw_name_2_keyword.clear();
        self.flags = 0_i32;
    }
}
