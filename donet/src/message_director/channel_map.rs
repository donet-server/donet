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

use libdonet::globals::Channel;
use multimap::MultiMap;
use std::ops::Range;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

#[derive(Clone, PartialEq)]
pub struct ChannelSubscriber {
    pub subscribed_channels: Vec<Channel>,
    pub subscribed_ranges: Range<Channel>,
}

pub type ChannelSubscriberRef = Arc<Mutex<ChannelSubscriber>>;

#[derive(Default)]
pub struct ChannelMap {
    subscriptions: MultiMap<Channel, ChannelSubscriberRef>,
    range_subscriptions: MultiMap<Channel, ChannelSubscriberRef>,
}

impl ChannelMap {
    /// Adds a single channel to the subscriber's subscribed channels map.
    pub async fn subscribe_channel(&mut self, sub: ChannelSubscriberRef, chan: Channel) {
        let mut locked_sub: MutexGuard<'_, ChannelSubscriber> = sub.lock().await;

        if self.is_subscribed(sub.clone(), chan).await {
            return;
        }
        locked_sub.subscribed_channels.push(chan);

        let has_subscriptions: bool = !locked_sub.subscribed_channels.is_empty();

        if has_subscriptions {
            // TODO: Implement 'on_add_channel' callback.
        }
        self.subscriptions.insert(chan, sub.clone());
    }

    /// Removes the given channel from the subscribed channels map.
    pub async fn unsubscribe_channel(&mut self, sub: ChannelSubscriberRef, chan: Channel) {
        let mut locked_sub: MutexGuard<'_, ChannelSubscriber> = sub.lock().await;

        if self.is_subscribed(sub.clone(), chan).await {
            return;
        }
        let mut index: usize = 0;

        for subscription in &locked_sub.subscribed_channels {
            if chan != *subscription {
                index += 1;
                continue;
            }
            break;
        }
        locked_sub.subscribed_channels.swap_remove(index);
    }

    /// Adds an object to be subscribed to a range of channels. The range is inclusive.
    pub fn subscribe_range(&mut self, _sub: ChannelSubscriberRef, _min: Channel, _max: Channel) {}

    /// Performs the reverse of the subscribe_range() method.
    pub fn unsubscribe_range(&mut self, _sub: ChannelSubscriberRef, _min: Channel, _max: Channel) {}

    /// Removes all channel and range subscriptions from the subscriber.
    pub fn unsubscribe_all(&mut self, _sub: ChannelSubscriberRef) {}

    /// Removes the given subscriber from the MultiMap for a given channel.
    ///
    /// Returns true only if:
    /// a) There are subscribers for the given channel and
    /// b) The provided subscriber was the last one for the channel, and was removed successfully.
    ///
    pub async fn remove_subscriber(&mut self, sub: ChannelSubscriberRef, chan: Channel) -> bool {
        let locked_sub: MutexGuard<'_, ChannelSubscriber> = sub.lock().await;
        let mut sub_count: usize = self.subscriptions.len();

        if sub_count == 0 {
            return false;
        }
        for (key, subscriptions) in self.subscriptions.iter_all_mut() {
            if *key != chan {
                continue;
            }
            let mut index: usize = 0;
            let mut found: bool = false;

            for subscription in subscriptions.iter_mut() {
                if *subscription.lock().await == *locked_sub {
                    found = true;
                    sub_count -= 1;
                }
                index += 1;
            }
            if found {
                // The swap_remove() function requires a second mutable
                // borrow of the vector which is illegal, so I had to work
                // around this by using a `found` flag and performing the
                // remove outside of the for loop which turns values into
                // an iterator. This way we don't perform a second borrow.
                subscriptions.swap_remove(index);
            }
        }
        sub_count == 0
    }

    /// Checks if a given object has a subscription on a channel.
    pub async fn is_subscribed(&mut self, sub: ChannelSubscriberRef, chan: Channel) -> bool {
        let locked_sub: MutexGuard<'_, ChannelSubscriber> = sub.lock().await;

        if locked_sub.subscribed_channels.contains(&chan) {
            return true;
        }
        if locked_sub.subscribed_ranges.contains(&chan) {
            return true;
        }
        false
    }

    /// Performs the same check as is_subscribed(), but for an array of channels.
    pub fn are_subscribed(&mut self, _subs: &mut [ChannelSubscriber], _chans: &[Channel]) {}
}
