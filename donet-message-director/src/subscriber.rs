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

use core::net::SocketAddr;
use donet_core::datagram::datagram::*;
use donet_core::globals::Channel;
use donet_network::Client;
use donet_network::HasClient;
use gcollections::ops::*;
use interval::IntervalSet;
use log::trace;
use multimap::MultiMap;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::{Mutex, MutexGuard};

/// A wrapper that holds a thread-safe [`std::sync::Arc`] pointer to
/// a [`Subscriber`] wrapped in a [`tokio::sync::Mutex`] that can
/// live across `.await` points.
///
/// This wrapper exists so it can manually implement the
/// [`core::cmp::Eq`] and [`std::hash::Hash`] traits, where only the
/// subscriber's remote IPv4/6 address is used for comparison of
/// references and to hash a reference, allowing this wrapper to be
/// stored in a [`std::collections::HashSet`].
///
/// The remote address is immutable and should never change, which
/// means the comparison and hash of this structure should always be
/// the same, satisfying the requirements for a hash set.
#[derive(Clone)]
pub struct SubscriberRef {
    hash_key: SocketAddr,
    pointer: Arc<Mutex<Subscriber>>,
}

impl From<Subscriber> for SubscriberRef {
    fn from(value: Subscriber) -> Self {
        Self {
            hash_key: value.remote,
            pointer: Arc::new(Mutex::new(value)),
        }
    }
}

/// Dummy [`SubscriberRef`] for finding in hash set.
///
/// Also used for unit testing, where we don't have real
/// [`Client`] structures to make a [`Subscriber`] from.
impl From<SocketAddr> for SubscriberRef {
    fn from(value: SocketAddr) -> Self {
        Self {
            hash_key: value,
            pointer: Arc::new(Mutex::new(value.into())),
        }
    }
}

/// Must implement [`core::cmp::PartialEq`] to store in a
/// [`std::collections::HashSet`] structure.
///
/// Compares the remote IPv4/6 address of both subscribers.
impl PartialEq for SubscriberRef {
    fn eq(&self, other: &Self) -> bool {
        self.hash_key == other.hash_key
    }
}

impl Eq for SubscriberRef {}

/// Must implement [`std::hash::Hash`] to store in a
/// [`std::collections::HashSet`] structure.
///
/// Hashes the raw pointer of the [`std::sync::Arc`].
impl std::hash::Hash for SubscriberRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash_key.hash(state);
    }
}

impl SubscriberRef {
    /// Thin wrapper of the [`tokio::sync::Mutex::lock`] function.
    pub async fn lock(&self) -> MutexGuard<'_, Subscriber> {
        self.pointer.lock().await
    }

    /// Get a clone of the underlying [`std::sync::Arc`]
    /// pointer to the [`Subscriber`] mutex.
    pub fn get_ptr(&self) -> Arc<Mutex<Subscriber>> {
        self.pointer.clone()
    }

    /// Quick way to get the remote address without locking
    /// the underlying [`Subscriber`]'s mutex.
    pub fn get_remote(&self) -> SocketAddr {
        self.hash_key
    }
}

/// Simple representation of a participant, or subscriber,
/// that is connected to a Message Director instance.
#[derive(Debug)]
pub struct Subscriber {
    /// Remote IPv4/6 address of this subscriber.
    ///
    /// This is the unique identifier for this subscriber.
    remote: SocketAddr,
    /// [`Client`] for this subscriber. Can be `None`, as
    /// a dummy [`Subscriber`] struct can be created for
    /// looking up a [`SubscriberRef`] in a hash set.
    client: Option<Arc<Mutex<Client>>>,
    /// The name for this downstream connection.
    pub connection_name: Option<String>,
    /// The web URL for this downstream connection.
    pub connection_web_url: Option<String>,
    /// Single channel subscriptions
    pub subscribed_channels: HashSet<Channel>,
    /// Channel range subscriptions
    pub subscribed_ranges: IntervalSet<Channel>,
    /// Datagrams scheduled to be distributed upon
    /// this subscriber's unexpected disconnect.
    pub post_removes: MultiMap<Channel, Datagram>,
}

/// Creates a new [`Subscriber`] from a [`SocketAddr`],
/// should be the remote address of the subscriber.
///
/// This can also be used to make a dummy
/// [`SubscriberRef`] for looking up a subscriber
/// in a hash set.
impl From<SocketAddr> for Subscriber {
    fn from(value: SocketAddr) -> Self {
        Self {
            client: None,
            remote: value,
            connection_name: None,
            connection_web_url: None,
            subscribed_channels: HashSet::default(),
            subscribed_ranges: IntervalSet::empty(),
            post_removes: MultiMap::default(),
        }
    }
}

impl PartialEq for Subscriber {
    fn eq(&self, other: &Self) -> bool {
        self.remote == other.remote
    }
}

impl HasClient for Subscriber {
    fn get_client(&self) -> Arc<Mutex<Client>> {
        self.client
            .as_ref()
            .expect("Tried to get client on dummy sub.")
            .clone()
    }
}

impl Subscriber {
    pub async fn new(client: Client) -> Self {
        Self {
            remote: client.get_remote(),
            client: Some(Arc::new(Mutex::new(client))),
            connection_name: None,
            connection_web_url: None,
            subscribed_channels: HashSet::default(),
            subscribed_ranges: IntervalSet::empty(),
            post_removes: MultiMap::default(),
        }
    }

    /// Handles a [`Datagram`] that the Message Director received,
    /// and needs to be routed to this subscriber.
    pub async fn handle_datagram(
        &mut self,
        dg: &mut Datagram,
    ) -> Result<(), mpsc::error::SendError<Datagram>> {
        trace!("Sending datagram downstream to {}", self.remote);

        debug_assert!(
            self.client.is_some(),
            "Called handle_datagram() on a dummy Subscriber!"
        );

        // if assertion passed, we can safely unwrap
        let client: Arc<Mutex<Client>> = self.client.clone().unwrap();
        let mut locked_client = client.lock().await;

        locked_client.stage_datagram(dg.clone()).await
    }

    pub async fn receive_disconnect(&mut self) {
        // TODO!
    }

    pub async fn post_remove(&mut self) {
        // TODO!
    }
}
