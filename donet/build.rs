// DONET SOFTWARE
// Copyright (c) 2024, Donet Authors.

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

use git_sha1::GitSHA1;

fn main() -> std::io::Result<()> {
    // setup for git-sha1 crate
    GitSHA1::read()
        .use_default("N/A (Compiled outside of Git)")
        .set("GIT_SHA1");
    Ok(())
}
