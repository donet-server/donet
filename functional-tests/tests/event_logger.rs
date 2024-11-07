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

use donet_core::datagram::datagram::Datagram;
use donet_daemon::config;
use donet_daemon::event::LoggedEvent;
use donet_daemon::service::{set_future_return_type, DonetService};
use donet_event_logger::EventLogger;
use donet_network::udp;

use tokio::runtime::{Builder, Runtime};
use tokio::task::{spawn, JoinHandle};
use tokio::time::{sleep, Duration};

static SERVICE_BIND_ADDRESS: &str = "127.0.0.1:19090";
static NETWORK_PROCESS_TIME: u64 = 1; // seconds

fn clean_up_logs() -> std::io::Result<()> {
    for entry in glob::glob("*.log").expect("Failed to read glob pattern") {
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
fn event_logger_functional_testing() -> std::io::Result<()> {
    let test_main = async move {
        // setup daemon configuration
        let conf: config::DonetConfig = config::DonetConfig {
            daemon: config::Daemon {
                name: String::default(),
                id: None,
                log_level: None,
            },
            global: config::Global {
                eventlogger: String::default(),
                dc_files: vec![],
                dc_multiple_inheritance: None,
                dc_sort_inheritance_by_file: None,
                dc_virtual_inheritance: None,
            },
            services: config::Services {
                event_logger: Some(config::EventLogger {
                    bind: SERVICE_BIND_ADDRESS.to_string(),
                    output: "./".to_string(),
                    log_format: "el-%Y-%m-%d-%H-%M-%S.log".to_string(),
                    rotate_interval: "1d".to_string(),
                }),
                client_agent: None,
                message_director: None,
                state_server: None,
                database_server: None,
                dbss: None,
            },
        };

        // Start the Event Logger service
        let service_handle = EventLogger::start(conf, None).await?;

        let mut task_handles: Vec<JoinHandle<_>> = vec![];

        // spawn blocking tasks for each test
        task_handles.push(spawn(basic_message_test()));

        // await test tasks to finish up, verify no tasks errored
        for handle in task_handles {
            assert!(handle.await.is_ok());
        }

        // stop the event logger service
        service_handle.abort();

        // verify that the service task's error is due to our abort
        assert!(service_handle.await.unwrap_err().is_cancelled());

        Ok(())
    };

    // Hack to reassure the compiler that I want to return an IO result.
    set_future_return_type::<std::io::Result<()>, _>(&test_main);

    let tokio_runtime: Runtime = Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .thread_stack_size(2 * 1024 * 1024) // default: 2MB
        .build()?;

    tokio_runtime.block_on(test_main)?;

    clean_up_logs()
}

async fn basic_message_test() -> std::io::Result<()> {
    let sock: udp::Socket = udp::Socket::bind("127.0.0.1:2816").await?;
    let dg: Datagram;

    let mut new_log = LoggedEvent::new("test", "Unit Test Socket");
    new_log.add("msg", "This is a test log message.");

    dg = new_log.make_datagram();

    sock.socket.send_to(&dg.get_data(), SERVICE_BIND_ADDRESS).await?;

    sleep(Duration::from_secs(NETWORK_PROCESS_TIME)).await;

    Ok(())
}
