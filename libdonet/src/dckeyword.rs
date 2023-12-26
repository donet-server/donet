// DONET SOFTWARE
// Copyright (c) 2023, Donet Authors.
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

pub type HistoricalFlag = u16;

#[derive(Debug)]
pub struct DCKeyword {
    name: String,
    /* This flag is only kept for historical reasons, so we can
     * preserve the DC file's hash code if no new flags are in use.
     */
    historical_flag: HistoricalFlag,
}

pub trait DCKeywordInterface {
    fn new(name: String, historical_flag: Option<HistoricalFlag>) -> DCKeyword;
    fn get_name(&self) -> String;
    fn get_historical_flag(&self) -> HistoricalFlag;
    fn clear_historical_flag(&mut self);
}

impl DCKeywordInterface for DCKeyword {
    fn new(name: String, historical_flag: Option<HistoricalFlag>) -> DCKeyword {
        if let Some(h_flag) = historical_flag {
            DCKeyword {
                name: name,
                historical_flag: h_flag,
            }
        } else {
            DCKeyword {
                name: name,
                historical_flag: !0, // bitwise complement
            }
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_historical_flag(&self) -> HistoricalFlag {
        self.historical_flag.clone()
    }

    fn clear_historical_flag(&mut self) {
        self.historical_flag = !0;
    }
}
