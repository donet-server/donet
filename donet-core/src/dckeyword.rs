/*
    This file is part of Donet.

    Copyright © 2024-2025 Max Rodriguez <me@maxrdz.com>

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

#[derive(Debug, PartialEq, Eq)]
pub struct DCKeyword(String);

impl std::fmt::Display for DCKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "keyword ")?;
        f.write_str(&self.0)?;
        write!(f, ";")
    }
}

impl LegacyDCHash for DCKeyword {
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        hashgen.add_string(self.0.clone());
    }
}

impl DCKeyword {
    #[inline]
    pub fn get_name(&self) -> String {
        self.0.clone()
    }
}

/// Represents the two types of inputs that `DCKeywordList.has_keyword`
/// accepts for looking up a Keyword. In Panda and Astron, the
/// `has_keyword` method is overloaded instead.
pub enum IdentifyKeyword {
    ByStruct(DCKeyword),
    ByName(String),
}

/// This is a list of [`DCKeyword`] structures, which represent
/// communication keywords that may be set on a particular field.
#[derive(Debug)]
pub struct DCKeywordList {
    keywords: Vec<DCKeyword>,
}
/* TODO!
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
}*/

impl std::fmt::Display for DCKeywordList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, kw) in self.keywords.iter().enumerate() {
            // We do not call the fmt::Display impl of [`DCKeyword`] here,
            // as that formats it as a declaration, not use in a field's
            // keyword list. So, we just need to format the kw identifier.
            f.write_str(&kw.0)?;

            if i != self.keywords.len() - 1 {
                write!(f, " ")?;
            }
        }
        writeln!(f, ";")
    }
}

impl LegacyDCHash for DCKeywordList {
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        hashgen.add_int(self.keywords.len().try_into().unwrap());

        for keyword in &self.keywords {
            keyword.generate_hash(hashgen);
        }
    }
}

impl DCKeywordList {
    /// Returns the number of keywords in this keyword list.
    pub fn get_num_keywords(&self) -> usize {
        self.keywords.len()
    }

    /// Returns `true` if given keyword identifier or struct
    /// is present in this keyword list.
    pub fn has_keyword(&self, kw: IdentifyKeyword) -> bool {
        match kw {
            IdentifyKeyword::ByName(_kw_id) => todo!("TODO!"),
            IdentifyKeyword::ByStruct(kw_obj) => {
                for keyword in &self.keywords {
                    if *keyword == kw_obj {
                        return true;
                    }
                }
                false // no match found
            }
        }
    }

    /// Returns [`DCKeyword`] reference by index, wrapped in an Option.
    pub fn get_keyword(&self, index: usize) -> Option<&DCKeyword> {
        self.keywords.get(index)
    }
}

pub(crate) mod semantics {
    /* TODO!
    pub fn add_keyword(element: &mut DCKeywordList, keyword: DCKeyword) -> Result<(), ()> {
        let kw_name: String = keyword.name.clone(); // avoid moving 'name'

        if self.kw_name_2_keyword.get(&kw_name).is_some() {
            return Err(()); // keyword is already in our list!
        }

        self.keywords.push(Rc::new(keyword));
        self.kw_name_2_keyword
            .insert(kw_name, self.keywords.last().unwrap().clone());
        Ok(())
    }

    /// Overwrites the DCKeywords of this list with the target's DCKeywords.
    pub fn copy_keywords(element: &mut DCKeywordList, target: &DCKeywordList) {
        let target_kw_array: Vec<Rc<DCKeyword>> = target._get_keyword_list();
        let target_kw_map: MultiMap<String, Rc<DCKeyword>> = target._get_keywords_by_name_map();

        self.keywords = target_kw_array; // old vec will be dropped from memory
        self.kw_name_2_keyword = target_kw_map;
    }*/
}
