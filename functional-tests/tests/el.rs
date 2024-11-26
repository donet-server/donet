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

//! Functional testing for the Event Logger service of
//! the Donet server.
//!
//! The TOML configuration file used for the daemon is
//! located in a file named "el.toml" in this directory.

use donet_core::datagram::datagram::*;
use donet_daemon::event::*;
use std::env;
use std::net::UdpSocket;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

static DAEMON_BIN: &str = "donetd";
static DAEMON_TOML: &str = "el.toml";

/// Must be the same as the one found in the TOML!
static SERVICE_BIND_ADDR: &str = "127.0.0.1:19090";

static NETWORK_PROCESS_TIME: u64 = 200; // milliseconds

fn clean_up_logs(pwd: String) -> std::io::Result<()> {
    for entry in glob::glob(&format!("{}/*.log", pwd)).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if path.is_file() {
                    std::fs::remove_file(path)?;
                }
            }
            Err(e) => println!("Error processing file pattern: {}", e),
        }
    }
    Ok(())
}

#[test]
fn el_functional_testing() -> std::io::Result<()> {
    let build_dir: String =
        env::var("MESON_BUILD_ROOT").expect("Functional tests need to be ran through Meson.");

    let src_dir: String =
        env::var("MESON_SOURCE_ROOT").expect("Functional tests need to be ran through Meson.");

    let pwd: String = format!("{}/functional-tests/tests", src_dir);

    // clean up any logs from previous test runs
    clean_up_logs(pwd.clone())?;

    let mut donet = Command::new(format!("{}/{}", build_dir, DAEMON_BIN))
        .current_dir(pwd)
        .arg(DAEMON_TOML)
        .spawn()
        .expect("Donet daemon failed to launch.");

    sleep(Duration::from_millis(NETWORK_PROCESS_TIME));

    // bind new UDP socket to any port (OS allocates the port for us)
    let sock = std::net::UdpSocket::bind("127.0.0.1:0")?;

    match sock.connect(SERVICE_BIND_ADDR) {
        Ok(_) => {}
        Err(err) => {
            donet.kill()?;
            panic!("Could not connect to the event logger.: {}", err);
        }
    }

    basic_log_msg_test(&sock)?;

    assert!(donet.try_wait().unwrap().is_none(), "Test failed.");

    donet.kill()
}

fn basic_log_msg_test(sock: &UdpSocket) -> std::io::Result<()> {
    let dg: Datagram;

    let mut new_log = LoggedEvent::new("test", "Unit Test Socket");
    new_log.add("msg", "This is a test log message.");

    dg = new_log.make_datagram();

    sock.send(&dg.get_data())?;

    sleep(Duration::from_millis(NETWORK_PROCESS_TIME));
    Ok(())
}
