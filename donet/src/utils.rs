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

use std::error::Error;
use std::future::Future;

// MySQL Result (mysql crate API response)
pub type SqlResult = Result<(), Box<dyn Error>>;

// Hack to reassure the compiler the result type of a future.
pub fn set_future_return_type<T, F: Future<Output = T>>(_arg: &F) {}

#[cfg(test)]
mod unit_testing {
    use super::set_future_return_type;
    use std::io::Result;

    #[test]
    fn test_future_return_type_util() {
        let test_future = async move {
            println!("async!");
            Ok(())
        };
        // Just make sure it doesn't panic or anything goofy.
        // Need this test to have test coverage on this file.
        set_future_return_type::<Result<()>, _>(&test_future);
    }
}
