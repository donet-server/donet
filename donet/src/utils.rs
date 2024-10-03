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
