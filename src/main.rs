// PANDANET SOFTWARE
// Copyright (c) 2023, PandaNet Authors.

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

#[path = "datagram.rs"]
mod datagram;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let _option: &String = &args[1];
    }
    println!("Hello, world!");
}
