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

//! Functional testing for the Message Director service of
//! the Donet server.
//!
//! The TOML configuration file used for the daemon is
//! located in a file named "md.toml" in this directory.

use donet_core::datagram::datagram::*;
use donet_core::globals::*;
use donet_core::Protocol;
use std::env;
use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::process::{Child, Command};
use std::thread::sleep;
use std::time::Duration;

static DAEMON_BIN: &str = "donetd";
static DAEMON_TOML: &str = "md.toml";

/// Must be the same as the one found in the TOML!
static SERVICE_BIND_ADDR: &str = "127.0.0.1:57123";

static NETWORK_PROCESS_TIME: u64 = 100; // milliseconds
static TCP_READ_TIMEOUT: u64 = 100; // milliseconds
static TCP_READ_BUFFER_SIZE: usize = 206; // bytes

/// Perform an `assert_eq!` where we clean up the spawned
/// Donet daemon process before actually panicking.
///
/// A [`Child`] process does not kill itself on drop,
/// so we kill it manually here.
macro_rules! clean_assert_eq {
    ($proc:expr, $lhs:expr, $rhs:expr) => {
        // check if the assertion would fail
        if $lhs != $rhs {
            // it will. kill our child processes
            kill_procs($proc)?;
            assert_eq!($lhs, $rhs);
        }
    };
    // rule for optional string literal as assert message
    ($proc:expr, $lhs:expr, $rhs:expr, $str:tt) => {
        if $lhs != $rhs {
            kill_procs($proc)?;
            assert_eq!($lhs, $rhs, $str);
        }
    };
}

/// Write to a TCP stream without the risk of leaving the
/// Donet daemon running if an IO error is returned.
macro_rules! clean_sock_write_all {
    ($proc:expr, $sock:expr, $bytes:expr) => {
        if let Err(e) = $sock.write_all($bytes) {
            kill_procs($proc)?;
            return Err(e);
        }
    };
}

/// Kills child processes before panicking.
macro_rules! clean_panic {
    ($proc:expr, $str:tt) => {
        // curly braces are a hack to stop the compiler
        // from crying about the '?' operator. Just make
        // sure you're this macro expands in a function
        // that returns an [`std::io::Result`] type.
        {
            kill_procs($proc)?;
            panic!($str);
        }
    };
    ($proc:expr, $str:tt, $($f:expr),*) => {
        {
            kill_procs($proc)?;
            panic!($str, $($f),*);
        }
    }
}

/// Reads from a TCP stream without the risk of leaving the
/// Donet daemon running if an IO error is returned.
///
/// Also handles retrying reads if an IO error is returned
/// at first.
fn clean_sock_read(
    procs: &mut Vec<Child>,
    sock: &mut TcpStream,
    read_buf: &mut [u8],
) -> std::io::Result<usize> {
    let mut i: usize = 0;
    loop {
        match sock.read(read_buf) {
            Ok(read) => {
                return Ok(read);
            }
            Err(err) => {
                match err.kind() {
                    // On Unix-like, [`ErrorKind::WouldBlock`] *can* happen in a
                    // socket that is in blocking mode, if a read timeout is set (which is).
                    //
                    // On Windows, reaching the read timeout would return `TimedOut`.
                    ErrorKind::WouldBlock | ErrorKind::TimedOut | ErrorKind::Interrupted => {
                        if i != 5 {
                            eprintln!("Tried to read from sock, got: {}; Retrying.", err);

                            sleep(Duration::from_millis(NETWORK_PROCESS_TIME));
                            i += 1;
                            continue;
                        }
                        kill_procs(procs)?;
                        return Err(err);
                    }
                    _ => clean_panic!(procs, "Got unexpected IO error: {}", err),
                }
            }
        }
    }
}

/// Utility function for killing all spawned [`Child`] processes.
fn kill_procs(procs: &mut Vec<Child>) -> std::io::Result<()> {
    for proc in procs.iter_mut() {
        proc.kill()?;
    }
    Ok(())
}

#[test]
fn md_functional_testing() -> std::io::Result<()> {
    let build_dir: String =
        env::var("MESON_BUILD_ROOT").expect("Functional tests need to be ran through Meson.");

    let src_dir: String =
        env::var("MESON_SOURCE_ROOT").expect("Functional tests need to be ran through Meson.");

    let pwd: String = format!("{}/functional-tests/tests", src_dir);

    let mut procs: Vec<Child> = vec![];

    procs.push(
        Command::new(format!("{}/{}", build_dir, DAEMON_BIN))
            .current_dir(pwd)
            .arg(DAEMON_TOML)
            .spawn()
            .expect("Donet daemon failed to launch."),
    );

    sleep(Duration::from_millis(NETWORK_PROCESS_TIME));

    // setup our TCP socket to interact with the MD as a subscriber
    let mut sock = match TcpStream::connect(SERVICE_BIND_ADDR) {
        Ok(sock) => sock,
        Err(err) => clean_panic!(&mut procs, "Could not connect to the message director.: {}", err),
    };
    sock.set_nonblocking(false)
        .expect("set_nonblocking() call failed");

    sock.set_read_timeout(Some(Duration::from_millis(TCP_READ_TIMEOUT)))?;

    // run functional tests
    test_add_channels(&mut procs, &mut sock)?;
    test_add_range(&mut procs, &mut sock)?;

    // all tests ran without panicking or returning an error, so lets
    // finally verify that the donet daemon is still standing
    let donet: &mut Child = procs.get_mut(0).expect("Donet process should be found.");

    assert!(donet.try_wait().unwrap().is_none(), "Daemon crashed.");
    donet.kill()
}

fn test_add_channels(procs: &mut Vec<Child>, sock: &mut TcpStream) -> std::io::Result<()> {
    eprintln!("test_add_channels()");

    // subscribe to a channel
    let mut dg: Vec<u8> = msgs::add_channel(401000000);
    dg.append(&mut msgs::add_channel(402000000));

    clean_sock_write_all!(procs, sock, &dg);
    sleep(Duration::from_millis(NETWORK_PROCESS_TIME));

    // send test message to our own subscribed channel
    let mut test_dg: Datagram = Datagram::default();
    test_dg.add_size(17 + 2).unwrap();
    test_dg
        .add_internal_header(vec![401000000], 1337, Protocol::CAAddInterest.into())
        .unwrap();

    let test_dg_raw: &[u8] = test_dg.get_buffer();

    clean_sock_write_all!(procs, sock, &test_dg_raw);
    sleep(Duration::from_millis(NETWORK_PROCESS_TIME));

    // MD should bounce that message back to us, since we sent it to our
    // channel that we added, so lets read it and verify its integrity
    let mut read_buf = [0_u8; TCP_READ_BUFFER_SIZE];

    let bytes_read: usize = clean_sock_read(procs, sock, &mut read_buf)?;
    eprintln!("{:?}", read_buf);

    clean_assert_eq!(
        procs,
        bytes_read,
        test_dg.size(),
        "did not receive expected number of bytes. may have also reached read timeout."
    );

    // convert read buffer to a `Vec<u8>` and truncate to `bytes_read` size.
    let mut read_vec: Vec<u8> = read_buf.to_vec();
    read_vec.truncate(bytes_read);

    clean_assert_eq!(procs, read_vec, test_dg_raw, "did not receive expected datagram");

    Ok(())
}

fn test_add_range(procs: &mut Vec<Child>, sock: &mut TcpStream) -> std::io::Result<()> {
    eprintln!("test_add_range()");

    // subscribe to a range of channels
    let dg: Vec<u8> = msgs::add_range(4000..5000);

    clean_sock_write_all!(procs, sock, &dg);
    sleep(Duration::from_millis(NETWORK_PROCESS_TIME));

    for channel in 4000..5000 {
        // just test by 100s (4000, 4100, 4200, ...)
        if channel % 100 != 0 {
            continue;
        }
        // send test message to our own subscribed channel
        let mut test_dg: Datagram = Datagram::default();
        test_dg.add_size(17 + 2).unwrap();
        test_dg
            .add_internal_header(vec![channel], 1337, Protocol::SSObjectSetOwner.into())
            .unwrap();

        let test_dg_raw: &[u8] = test_dg.get_buffer();

        clean_sock_write_all!(procs, sock, &test_dg_raw);
        sleep(Duration::from_millis(NETWORK_PROCESS_TIME));

        // MD should bounce that message back to us, since we sent it to our
        // channel that we added, so lets read it and verify its integrity
        let mut read_buf = [0_u8; TCP_READ_BUFFER_SIZE];

        let bytes_read: usize = clean_sock_read(procs, sock, &mut read_buf)?;
        eprintln!("{:?}", read_buf);

        clean_assert_eq!(
            procs,
            bytes_read,
            test_dg.size(),
            "did not receive expected number of bytes. may have also reached read timeout."
        );

        // convert read buffer to a `Vec<u8>` and truncate to `bytes_read` size.
        let mut read_vec: Vec<u8> = read_buf.to_vec();
        read_vec.truncate(bytes_read);

        clean_assert_eq!(procs, read_vec, test_dg_raw, "did not receive expected datagram");
    }

    Ok(())
}

mod msgs {
    use super::*;

    pub fn add_channel(channel: Channel) -> Vec<u8> {
        let mut dg = Datagram::default();

        // manually write size tag, as we're rawdogging datagrams.
        // in Donet source, `donet_network::Client` handles this.
        //
        // A control header is always 11 bytes:
        //
        //  - 1 byte for the recipients
        //  - 8 bytes for the recipient channel (control channel)
        //  - 2 bytes for the control message type
        dg.add_size(11 + 8).unwrap();
        dg.add_control_header(Protocol::MDAddChannel.into()).unwrap();

        dg.add_channel(channel).unwrap();
        dg.get_data()
    }

    pub fn add_range(range: std::ops::Range<Channel>) -> Vec<u8> {
        let mut dg = Datagram::default();

        // control header, 2 channels
        dg.add_size(11 + 8 + 8).unwrap();
        dg.add_control_header(Protocol::MDAddRange.into()).unwrap();

        dg.add_channel(range.start).unwrap();
        dg.add_channel(range.end).unwrap();
        dg.get_data()
    }

    pub fn remove_channel(channel: Channel) -> Vec<u8> {
        let mut dg = Datagram::default();

        // control header, 1 channel
        dg.add_size(11 + 8).unwrap();
        dg.add_control_header(Protocol::MDRemoveChannel.into()).unwrap();

        dg.add_channel(channel).unwrap();
        dg.get_data()
    }

    pub fn remove_range(range: std::ops::Range<Channel>) -> Vec<u8> {
        let mut dg = Datagram::default();

        // control header, 2 channels
        dg.add_size(11 + 8 + 8).unwrap();
        dg.add_control_header(Protocol::MDRemoveRange.into()).unwrap();

        dg.add_channel(range.start).unwrap();
        dg.add_channel(range.end).unwrap();
        dg.get_data()
    }

    pub fn add_post_remove(sender: Channel, datagram: Datagram) -> Vec<u8> {
        let mut dg = Datagram::default();

        // control header, 1 channel, 1 blob
        dg.add_size(11 + 8 + 2 + datagram.size() as DgSizeTag).unwrap();
        dg.add_control_header(Protocol::MDAddPostRemove.into()).unwrap();

        dg.add_channel(sender).unwrap();
        dg.add_blob(datagram.get_data()).unwrap();
        dg.get_data()
    }

    pub fn clear_post_removes(sender: Channel) -> Vec<u8> {
        let mut dg = Datagram::default();

        // control header, 1 channel
        dg.add_size(11 + 8).unwrap();
        dg.add_control_header(Protocol::MDClearPostRemoves.into())
            .unwrap();

        dg.add_channel(sender).unwrap();
        dg.get_data()
    }

    pub fn log_message(msgpack_dg: Datagram) -> Vec<u8> {
        let mut dg = Datagram::default();

        // control header, 1 blob
        dg.add_size(11 + 2 + msgpack_dg.size() as DgSizeTag).unwrap();
        dg.add_control_header(Protocol::MDLogMessage.into()).unwrap();

        dg.add_blob(msgpack_dg.get_data()).unwrap();
        dg.get_data()
    }
}
