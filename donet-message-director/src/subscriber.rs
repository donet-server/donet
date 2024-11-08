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

use donet_core::globals::Channel;
use donet_network::Client;
use gcollections::ops::*;
use interval::IntervalSet;
use std::collections::HashSet;
use std::io::Result;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::{Mutex, MutexGuard};

/// A wrapper that holds a thread-safe [`std::sync::Arc`] pointer to
/// a [`Subscriber`] wrapped in a [`tokio::sync::Mutex`] that can
/// live across `.await` points.
///
/// This wrapper exists so it can manually implement the
/// [`core::cmp::Eq`] and [`std::hash::Hash`] traits, where only the
/// raw pointer of the underlying [`std::sync::Arc`] is used for
/// comparison of references and to hash a reference, allowing this
/// wrapper to be stored in a [`std::collections::HashSet`].
///
/// The [`std::sync::Arc`]'s inner pointer should stay the same,
/// which means the comparison and hash of this structure should
/// always be the same, satisfying the requirements for a hash set.
#[derive(Clone)]
pub struct SubscriberRef {
    pointer: Arc<Mutex<Subscriber>>,
}

impl From<Subscriber> for SubscriberRef {
    fn from(value: Subscriber) -> Self {
        Self {
            pointer: Arc::new(Mutex::new(value)),
        }
    }
}

/// Must implement [`core::cmp::PartialEq`] to store in a
/// [`std::collections::HashSet`] structure.
///
/// Compares the raw pointers of both [`std::sync::Arc`] pointers.
impl PartialEq for SubscriberRef {
    fn eq(&self, other: &Self) -> bool {
        self.get_ptr_address() == other.get_ptr_address()
    }
}

impl Eq for SubscriberRef {}

/// Must implement [`std::hash::Hash`] to store in a
/// [`std::collections::HashSet`] structure.
///
/// Hashes the raw pointer of the [`std::sync::Arc`].
impl std::hash::Hash for SubscriberRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.get_ptr_address());
    }
}

impl SubscriberRef {
    /// Thin wrapper of the [`tokio::sync::Mutex::lock`] function.
    pub async fn lock(&self) -> MutexGuard<'_, Subscriber> {
        self.pointer.lock().await
    }

    /// Returns the raw pointer of the underlying [`std::sync::Arc`].
    ///
    /// We do not need to dereference this raw pointer, so simply
    /// retrieving it and converting it to a `usize` is safe.
    pub fn get_ptr_address(&self) -> usize {
        Arc::as_ptr(&self.pointer) as *const () as usize
    }
}

/// Simple representation of a participant, or subscriber,
/// that is connected to a Message Director instance.
pub struct Subscriber {
    client: Client,
    /// Single channel subscriptions
    pub subscribed_channels: HashSet<Channel>,
    /// Channel range subscriptions
    pub subscribed_ranges: IntervalSet<Channel>,
}

impl PartialEq for Subscriber {
    fn eq(&self, other: &Self) -> bool {
        (self.subscribed_channels == other.subscribed_channels)
            && (self.subscribed_ranges == other.subscribed_ranges)
    }
}

impl Subscriber {
    pub async fn new(socket: TcpStream) -> Result<Self> {
        Ok(Self {
            client: Client::new(socket).await?,
            subscribed_channels: HashSet::default(),
            subscribed_ranges: IntervalSet::empty(),
        })
    }
}
