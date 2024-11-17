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

use crate::config;
use std::future::Future;
use std::io::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
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
///
/// A service's main data structure, [`Self::Service`], **must**
/// be wrapped in a [`std::sync::Arc`] and [`tokio::sync::Mutex`], as
/// it may split certain behavior into separate tokio tasks that all
/// need a reference to [`Self::Service`].
pub trait DonetService {
    type Service;
    type Configuration;

    fn create(
        conf: Self::Configuration,
        dc: Option<DCFile<'static>>,
    ) -> impl Future<Output = Result<Arc<Mutex<Self::Service>>>> + Send;

    fn start(
        conf: config::DonetConfig,
        dc: Option<DCFile<'static>>,
    ) -> impl Future<Output = Result<JoinHandle<Result<()>>>> + Send;

    /// This service's main asynchronous loop.
    fn main(service: Arc<Mutex<Self::Service>>) -> impl Future<Output = Result<()>> + Send;

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
mod tests {
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
