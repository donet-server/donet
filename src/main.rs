// DONET SOFTWARE
// Copyright (c) 2023, DoNet Authors.

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

#[path = "config.rs"]
mod config;
#[path = "logger.rs"]
mod logger;
#[path = "service_factory.rs"]
mod service_factory;

fn main() -> std::io::Result<()> {
    use self::logger::logger;
    use git_sha1::GitSHA1;
    use log::SetLoggerError;
    use std::fs::File;

    const VERSION_STRING: &str = "0.1.0";
    const CONFIG_FILE: &str = "daemon.toml";
    static GIT_SHA1: &str = env!("GIT_SHA1");
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        for argument in args {
            if argument == "-h" || argument == "--help" {
                println!(
                    "Usage:    donet [options] ... [CONFIG_FILE]\n\
                    \n\
                    DoNet - Distributed Object Network Engine.\n\
                    This binary will look for a configuration file (.toml)\n\
                    in the current working directory as \"{}\".\n\
                    \n\
                    -h, --help      Print the help page.\n\
                    -v, --version   Print DoNet binary version & build info.\n",
                    CONFIG_FILE
                );
                return Ok(());
            } else if argument == "-v" || argument == "--version" {
                let bin_arch: &str = if cfg!(target_arch = "x86") {
                    "x86"
                } else if cfg!(target_arch = "x86_64") {
                    "x86_64"
                } else if cfg!(target_arch = "aarch64") {
                    "aarch64"
                } else {
                    "unknown" // aka not supported
                };
                let bin_platform: &str = if cfg!(target_os = "linux") {
                    "linux"
                } else if cfg!(target_os = "windows") {
                    "windows"
                } else if cfg!(target_os = "macos") {
                    "macos"
                } else if cfg!(target_os = "freebsd") {
                    "freebsd"
                } else {
                    "unknown" // aka not supported
                };
                let bin_env: &str = if cfg!(target_env = "gnu") {
                    "gnu"
                } else if cfg!(target_env = "msvc") {
                    "msvc"
                } else {
                    "other"
                };
                println!(
                    "DoNet, version {} ({} {}-{})\n\
                    Revision (Git SHA1): {}\n\n\
                    Released under the AGPL-3.0 license. <https://www.gnu.org/licenses/agpl-3.0.html>\n\
                    View the source code on GitHub: https://github.com/donet-server/DoNet\n",
                    VERSION_STRING, bin_arch, bin_platform, bin_env, GIT_SHA1
                );
                return Ok(());
            }
        }
    }
    // Initialize the logger utility
    let res: Result<(), SetLoggerError> = logger::initialize_logger();
    if res.is_err() {
        panic!("Failed to initialize the logger utility!");
    }

    // Read the daemon configuration file
    let mut conf_file = File::open(CONFIG_FILE)?;
    return Ok(());
}
