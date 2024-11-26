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

//! This test is a functional test for the donet-core DC parser.
//!
//! The DC file and TOML configuration that is used in this
//! test is located in this directory.

use std::env;
use std::process::Command;

static DAEMON_BIN: &str = "donetd";
static DAEMON_TOML: &str = "dc_file.toml";
static DC_FILE: &str = "dc_file.dc";

#[test]
fn dc_language_functional_testing() {
    // When you run `meson compile tests`, Meson calls `cargo test`
    // with some environment variables that Meson sets to all targets
    // by default. We use these to get the path to the built server binary.
    let build_dir: String =
        env::var("MESON_BUILD_ROOT").expect("Functional tests need to be ran through Meson.");

    let src_dir: String =
        env::var("MESON_SOURCE_ROOT").expect("Functional tests need to be ran through Meson.");

    let pwd: String = format!("{}/functional-tests/tests", src_dir);

    let mut donet = Command::new(format!("{}/{}", build_dir, DAEMON_BIN))
        .current_dir(pwd)
        .arg("-c")
        .arg(DC_FILE)
        .arg(DAEMON_TOML)
        .spawn()
        .expect("Donet daemon failed to launch.");

    assert!(donet.wait().unwrap().success(), "Test failed.");
}
