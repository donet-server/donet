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

use crate::config;
use std::future::Future;
use std::io::Result;
use tokio::task::JoinHandle;

cfg_if! {
    if #[cfg(feature = "requires_dc")] {
        pub use donet_core::dcfile::DCFile;
    } else {
        /// Dummy DCFile struct for the [`DonetService`] trait
        /// to use on builds that do not require the DC file.
        ///
        /// This struct should never be initialized, as services
        /// that do not require the DC file will be passed `None`
        /// instead of `Some(DCFile)`.
        pub struct DCFile<'dc> {
            _hack: &'dc str, // required for lifetime parameter
        }
    }
}

/// Must be implemented by all Donet services in order to be
/// bootstrapped on daemon startup using this daemon's configuration.
pub trait DonetService {
    type Service;
    type Configuration;

    async fn create(conf: Self::Configuration, dc: Option<DCFile<'static>>) -> Result<Self::Service>;
    async fn start(conf: config::DonetConfig, dc: Option<DCFile<'static>>) -> Result<JoinHandle<Result<()>>>;

    /// This service's main asynchronous loop.
    async fn main(&mut self) -> Result<()>;

    /// Spawns a new Tokio asynchronous task that executes the given
    /// async function, and returns its Tokio join handle.
    fn spawn_async_task(
        service_loop: impl Future<Output = Result<()>> + Send + 'static,
    ) -> JoinHandle<Result<()>> {
        // Hack to reassure the compiler that we want to return an IO result.
        set_future_return_type::<Result<()>, _>(&service_loop);

        tokio::task::spawn(service_loop)
    }
}

/// Hack to reassure the compiler the result type of a future.
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
