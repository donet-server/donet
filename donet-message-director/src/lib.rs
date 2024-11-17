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

mod channel_map;
mod subscriber;
mod upstream;

use channel_map::*;
use core::net::SocketAddr;
use donet_core::datagram::datagram::*;
use donet_core::globals::*;
use donet_core::Protocol;
use donet_daemon::config;
use donet_daemon::service::*;
use donet_network::{tcp, udp};
use donet_network::{Client, HasClient, RecvData, RecvSendHandles};
use log::{error, info, trace, warn};
use std::collections::HashSet;
use std::io::{Error, ErrorKind, Result};
use std::sync::Arc;
use subscriber::*;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex, MutexGuard};
use tokio::task::JoinHandle;
use upstream::*;

/// Represents an internal protocol header.
///
/// Includes sender/recipient routing identifiers.
struct InternalHeader {
    sender: Channel,
    recipients: Vec<Channel>,
}

impl std::fmt::Display for InternalHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sender: {}, ", &self.sender.to_string())?;
        write!(f, "Recipients: {:?}", &self.recipients)
    }
}

/// Configuration data for this service.
///
/// We need some configuration from the `global` section of the TOML
/// as well, so we group both the MD config struct and any additional
/// data into this struct.
pub struct CreateInfo {
    service_conf: config::MessageDirector,
    event_logger_url: Option<String>,
}

pub struct MessageDirector {
    binding: Arc<Mutex<tcp::Acceptor>>,
    upstream_md: Option<UpstreamMD>,
    event_logger: Option<udp::Socket>,
    channel_map: ChannelMap,
    subscribers: HashSet<SubscriberRef>,
    removed_subscribers: HashSet<SubscriberRef>,
}

impl DonetService for MessageDirector {
    type Service = Self;
    type Configuration = CreateInfo;

    async fn create(
        conf: Self::Configuration,
        _: Option<DCFile<'static>>,
    ) -> Result<Arc<Mutex<Self::Service>>> {
        let bind_addr: &str = conf.service_conf.bind.as_str();
        let upstream: Option<String> = conf.service_conf.upstream;
        let logger_uri: Option<String> = conf.event_logger_url;

        Ok(Arc::new(Mutex::new(MessageDirector {
            binding: Arc::new(Mutex::new(tcp::Acceptor::bind(bind_addr).await?)),
            upstream_md: {
                match upstream {
                    Some(md_uri) => {
                        info!("Message Director will connect to upstream MD.");
                        Some(UpstreamMD::connect(&md_uri).await?)
                    }
                    None => None,
                }
            },
            event_logger: {
                match logger_uri {
                    Some(uri) => {
                        // requesting bind port '0' lets OS allocate a port for us
                        let mut new_sock = udp::Socket::bind("0.0.0.0:0").await?;

                        // have this new UDP socket send packets to
                        // the event logger's UDP socket bind address
                        new_sock.connect(&uri).await?;

                        Some(new_sock)
                    }
                    None => None,
                }
            },
            channel_map: ChannelMap::default(),
            subscribers: HashSet::default(),
            removed_subscribers: HashSet::default(),
        })))
    }

    async fn start(conf: config::DonetConfig, _: Option<DCFile<'static>>) -> Result<JoinHandle<Result<()>>> {
        let service_conf: CreateInfo = CreateInfo {
            // We can unwrap safely here since this function only is called if it is `Some`.
            service_conf: conf.services.message_director.unwrap(),
            event_logger_url: conf.global.eventlogger,
        };

        let service = MessageDirector::create(service_conf, None).await?;

        Ok(Self::spawn_async_task(async move {
            MessageDirector::main(service).await
        }))
    }

    async fn main(service: Arc<Mutex<Self::Service>>) -> Result<()> {
        // create a new mpsc channel for receiving incoming packets
        let (tx, mut rx) = mpsc::channel::<RecvData>(32);

        let service_clone_for_recv = service.clone();

        // spawn a tokio task for handling received datagrams from
        // clients connected to this MD.
        //
        // each client spawns tasks for handling their TCP stream,
        // so the way we communicate across tasks is via [`mpsc::channel`].
        let handle: JoinHandle<Result<()>> = tokio::spawn(async move {
            while let Some(recv_data) = rx.recv().await {
                let mut locked_service = service_clone_for_recv.lock().await;

                if let Err(e) = locked_service.handle_datagram(recv_data).await {
                    warn!("Failed to handle received datagram: {}", e);
                }
            }
            todo!("unhandled error. MD incoming datagram receiver returned None.")
        });

        // if we have an uplink connection, spawn send/receive tokio tasks
        if let Some(upstream) = &service.lock().await.upstream_md {
            let client = upstream.get_client();
            let mut client_lock = client.lock().await;

            let handles = client_lock.spawn_recv_send_tasks(tx.clone()).await;
        }

        let binding: Arc<Mutex<tcp::Acceptor>> = service.lock().await.binding.clone();
        let binding_lock = binding.lock().await;

        // start the main loop (accepting new TCP connections)
        loop {
            // here, we keep the TCP binding locked. only this loop needs it
            match binding_lock.socket.accept().await {
                Ok((socket, address)) => {
                    info!("Received incoming connection from {}.", address);

                    let mut service_lock = service.lock().await;

                    // create a new [`Subscriber`] from the new TCP connection,
                    // and pass a clone of `tx` for receiving its datagrams
                    match service_lock.new_connection(socket, tx.clone()).await {
                        Ok((recv_handle, send_handle)) => {
                            trace!("Created new subscriber.");
                            // TODO! handle task joins
                        }
                        Err(err) => {
                            info!("Failed to accept subscriber {}: {}", address, err);
                        }
                    }
                }
                Err(socket_err) => error!("Failed to get client: {}", socket_err),
            }
        }
    }
}

impl HasChannelMap for MessageDirector {
    fn get_channel_map(&mut self) -> &mut ChannelMap {
        &mut self.channel_map
    }
}

impl ChannelCoordinator for MessageDirector {
    async fn on_add_channel(&mut self, channel: Channel) {
        if let Some(upstream) = &mut self.upstream_md {
            upstream.stage_add_channel(channel).await;
        }
    }

    async fn on_add_range(&mut self, range: std::ops::Range<Channel>) {
        if let Some(upstream) = &mut self.upstream_md {
            upstream.stage_add_range(range).await;
        }
    }

    async fn on_remove_channel(&mut self, channel: Channel) {
        if let Some(upstream) = &mut self.upstream_md {
            upstream.stage_add_channel(channel).await;
        }
    }

    async fn on_remove_range(&mut self, range: std::ops::Range<Channel>) {
        if let Some(upstream) = &mut self.upstream_md {
            upstream.stage_remove_range(range).await;
        }
    }
}

impl MessageDirector {
    /// Allocates a new [`Subscriber`] in our hash set.
    async fn add_subscriber(&mut self, client: Client) -> Result<SubscriberRef> {
        // create a new [`Subscriber`] structure from the new client
        let sub: Subscriber = Subscriber::new(client).await;

        // move new subscriber struct to the heap and keep smart pointer
        let sub_ptr: SubscriberRef = sub.into();

        assert!(
            self.subscribers.insert(sub_ptr.clone()),
            "Subscriber already exists!"
        );
        Ok(sub_ptr)
    }

    /// Removes a [`Subscriber`] from our hash set using its
    /// remote IPv4/6 address ([`SocketAddr`]) as the key.
    async fn remove_subscriber(&mut self, remote: SocketAddr) -> Result<()> {
        match self.get_subscriber_with_remote(remote) {
            Some(sub_ref) => {
                // unsubscribe the subscriber from all its subscriptions
                self.unsubscribe_all(sub_ref.clone()).await;

                // stop tracking participant
                assert!(
                    self.subscribers.remove(&sub_ref),
                    "Tried to remove subscriber that doesn't exist.",
                );

                {
                    let mut locked_sub: MutexGuard<'_, Subscriber> = sub_ref.lock().await;

                    // Send out any post-remove messages the participant may have added.
                    // This is done last, because we don't want to send messages
                    // through the Director while a participant is being removed, as
                    // certain data structures may not have their invariants satisfied
                    // during that time.
                    locked_sub.post_remove().await;
                }

                // mark the subscriber for deletion
                self.removed_subscribers.insert(sub_ref);
                Ok(())
            }
            None => {
                warn!("Tried to remove subscriber that doesn't exist.");
                Ok(())
            }
        }
    }

    /// Takes in a [`SocketAddr`], returns a [`SubscriberRef`] or `None`.
    ///
    /// Retrieval can be done by creating a dummy [`SubscriberRef`]
    /// with the given [`SocketAddr`] value and calling the hashset's
    /// `get` function with the dummy [`SubscriberRef`].
    fn get_subscriber_with_remote(&self, remote: SocketAddr) -> Option<SubscriberRef> {
        self.subscribers.get(&remote.into()).cloned()
    }

    /// Creates a new [`Subscriber`] structure in memory from the
    /// new connected client, and spawns TCP stream handler tasks.
    async fn new_connection(
        &mut self,
        socket: TcpStream,
        tx: mpsc::Sender<RecvData>,
    ) -> Result<RecvSendHandles> {
        let client: Client = Client::from(socket);

        let sub_ptr: SubscriberRef = self.add_subscriber(client).await?;

        let sub = sub_ptr.get_ptr();
        let sub_lock = sub.lock().await;

        let client = sub_lock.get_client();

        let mut client_lock = client.lock().await;

        // start recv loop for subscriber client (connection)
        Ok(client_lock.spawn_recv_send_tasks(tx).await)
    }

    /// Entry point for all datagrams received from a client via their TCP socket.
    ///
    /// These datagrams can come from a subscriber (services or downstream MDs)
    /// or they can come from our upstream MD, if one is configured.
    async fn handle_datagram(&mut self, mut data: RecvData) -> Result<()> {
        trace!("Processing datagram ...");

        let recp_count: u8 = data.dgi.read_recipient_count();
        trace!("Recipient count: {}", recp_count);

        let mut recipients: Vec<Channel> = vec![];

        for _ in 0..recp_count {
            recipients.push(data.dgi.read_channel());
        }
        trace!("Recipient Channels: {:?}", recipients);

        // check if this is a control message
        if recp_count == 1 {
            if *recipients.first().unwrap() == CONTROL_CHANNEL {
                return self.handle_control_msg(data).await;
            }
        }

        // not a control msg, so there is a sender field ahead
        let sender: Channel = data.dgi.read_channel();

        // Store internal header info into struct
        let header = InternalHeader { sender, recipients };
        trace!("Datagram internal header: {}", &header);

        // route the regular internal message
        self.route_datagram(header, data).await.unwrap();
        Ok(())
    }

    /// Handles a datagram that is a CONTROL message, a.k.a it had one recipient
    /// and the recipient channel was the control channel (channel 1).
    async fn handle_control_msg(&mut self, mut data: RecvData) -> Result<()> {
        let msg_type: Protocol = data.dgi.read_msg_type();

        match msg_type {
            Protocol::MDAddChannel => {
                let channel: Channel = data.dgi.read_channel();
                let sub: SubscriberRef = self.get_subscriber_with_remote(data.remote).unwrap();

                self.subscribe_channel(sub, channel).await;
                Ok(())
            }
            Protocol::MDRemoveChannel => {
                let channel: Channel = data.dgi.read_channel();
                let sub: SubscriberRef = self.get_subscriber_with_remote(data.remote).unwrap();

                self.unsubscribe_channel(sub, channel).await;
                Ok(())
            }
            Protocol::MDAddRange => {
                let min: Channel = data.dgi.read_channel();
                let max: Channel = data.dgi.read_channel();

                let sub: SubscriberRef = self.get_subscriber_with_remote(data.remote).unwrap();

                self.subscribe_range(sub, min, max).await;
                Ok(())
            }
            Protocol::MDRemoveRange => {
                let min: Channel = data.dgi.read_channel();
                let max: Channel = data.dgi.read_channel();

                let sub: SubscriberRef = self.get_subscriber_with_remote(data.remote).unwrap();

                self.unsubscribe_range(sub, min, max).await;
                Ok(())
            }
            Protocol::MDAddPostRemove => {
                let sender: Channel = data.dgi.read_channel();
                let post_remove: Datagram = match data.dgi.read_datagram() {
                    Ok(dg) => dg,
                    Err(err) => {
                        warn!("Failed to read post remove datagram: {}", err);
                        return Ok(());
                    }
                };

                let sub: SubscriberRef = self.get_subscriber_with_remote(data.remote).unwrap();

                trace!("Subscriber with remote {} added a post remove.", sub.get_remote());

                sub.lock().await.post_removes.insert(sender, post_remove.clone());
                self.preroute_post_remove(sender, post_remove).await;
                Ok(())
            }
            Protocol::MDClearPostRemoves => {
                let sender: Channel = data.dgi.read_channel();

                let sub: SubscriberRef = self.get_subscriber_with_remote(data.remote).unwrap();

                trace!("Subscriber with remote {} added a post remove.", sub.get_remote());

                sub.lock().await.post_removes.remove(&sender);
                self.recall_post_removes(sender).await;
                Ok(())
            }
            Protocol::MDSetConName => {
                let con_name: String = data.dgi.read_string().unwrap();
                let sub: SubscriberRef = self.get_subscriber_with_remote(data.remote).unwrap();

                // Set the downstream connection's name
                sub.lock().await.connection_name = Some(con_name);
                Ok(())
            }
            Protocol::MDSetConUrl => {
                let con_web_url: String = data.dgi.read_string().unwrap();
                let sub: SubscriberRef = self.get_subscriber_with_remote(data.remote).unwrap();

                // Set the downstream connection's web URL
                sub.lock().await.connection_web_url = Some(con_web_url);
                Ok(())
            }
            Protocol::MDLogMessage => self.route_log_message(data).await,
            _ => {
                warn!(
                    "Received control message with a non-control message type from {}",
                    data.remote
                );
                // do not stop the MD, just log the error and resume
                Ok(())
            }
        }
    }

    /// Handles replicating and routing a datagram to its proper recipients
    /// based on this message director's channel subscriptions map.
    async fn route_datagram(
        &mut self,
        header: InternalHeader,
        mut data: RecvData,
    ) -> std::result::Result<(), impl std::error::Error> {
        let mut receiving_subscribers: HashSet<SubscriberRef> = HashSet::default();

        // get all subscribers of the recipient channels
        self.lookup_channels(header.recipients, &mut receiving_subscribers);

        // replicate the message to all receiving subscribers
        for sub in receiving_subscribers {
            sub.lock().await.handle_datagram(&mut data.dg).await.unwrap();
        }

        // Next, decide if this message needs to be routed **upstream**.
        //
        // First, we need to check if the sender of this message *is*
        // the upstream md. We do this by checking if it is a subscriber.
        let our_subscriber: bool = self.get_subscriber_with_remote(data.remote).is_some();

        // If the sender of this message is one of our subscribers
        // (downstream), **and** we have an uplink connection, route
        // the message upstream.
        if self.upstream_md.is_some() && our_subscriber {
            trace!("Routing upstream.");

            let upstream_lock = self.upstream_md.as_ref().unwrap();

            upstream_lock.stage_datagram(data.dg.clone()).await;
        } else if !our_subscriber {
            // If the sender's remote address does not match a subscriber in our hashset,
            // then this message is from upstream. Do not bounce it back!
            trace!("Not routing upstream; It came from there.");
        } else {
            // Otherwise, this is the master message director.
            trace!("Not routing upstream; We are the master MD.");
        }

        Ok::<(), mpsc::error::SendError<Datagram>>(())
    }

    /// Sends the post remove for the given sender by sending it
    /// upstream, if there is an upstream connection.
    async fn preroute_post_remove(&mut self, sender: Channel, post_remove: Datagram) {
        if let Some(upstream) = &mut self.upstream_md {
            upstream.stage_post_remove(sender, post_remove).await;
        }
    }

    /// Clears all post removes for the given sender by sending it
    /// upstream, if there is an upstream connection.
    async fn recall_post_removes(&mut self, sender: Channel) {
        if let Some(upstream) = &mut self.upstream_md {
            upstream.recall_post_removes(sender).await;
        }
    }

    /// Processes a CONTROL_LOG_MESSAGE message type and routes it to
    /// the appropriate event logger, either directly or uplink.
    async fn route_log_message(&mut self, mut data: RecvData) -> Result<()> {
        match &self.event_logger {
            Some(logger) => {
                let msgpack_blob_len: usize = data.dgi.get_remaining();

                let msgpack_payload = data.dgi.read_data(msgpack_blob_len);

                debug_assert!(msgpack_payload.is_ok(), "Tried to read past datagram.");

                let _: usize = logger.socket.send(&msgpack_payload.unwrap()).await?;
                Ok(())
            }
            None => {
                // We don't have a connection to the event logger, so
                // route the log control message upstream.
                match &mut self.upstream_md {
                    Some(upstream) => {
                        upstream.stage_datagram(data.dg.clone()).await;
                        Ok(())
                    }
                    // We don't have an upstream message director,
                    // so this log message simply will die with us.
                    None => {
                        warn!("CONTROL_LOG_MESSAGE received, but no event logger found.");
                        Ok(())
                    }
                }
            }
        }
    }
}
