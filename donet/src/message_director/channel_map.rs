// DONET SOFTWARE
// Copyright (c) 2024, Donet Authors.
//
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

use libdonet::globals::Channel;
use multimap::MultiMap;
use std::ops::Range;
use std::sync::{Arc, Mutex, MutexGuard};
use std::vec::Vec;

#[derive(Clone, PartialEq)]
pub struct ChannelSubscriber {
    pub subscribed_channels: Vec<Channel>,
    pub subscribed_ranges: Range<Channel>,
}

pub type ChannelSubscriberRef = Arc<Mutex<ChannelSubscriber>>;

pub struct ChannelMap {
    subscriptions: MultiMap<Channel, ChannelSubscriberRef>,
    _range_subscriptions: MultiMap<Channel, ChannelSubscriberRef>,
}

trait ChannelMapInterface {
    fn new() -> Self;
    // Adds a single channel to the subscriber's subscribed channels map.
    fn subscribe_channel(&mut self, sub: ChannelSubscriberRef, chan: Channel);
    // Removes the given channel from the subscribed channels map.
    fn unsubscribe_channel(&mut self, sub: ChannelSubscriberRef, chan: Channel);
    // Adds an object to be subscribed to a range of channels. The range is inclusive.
    fn subscribe_range(&mut self, _sub: ChannelSubscriberRef, _min: Channel, _max: Channel);
    // Performs the reverse of the subscribe_range() method.
    fn unsubscribe_range(&mut self, _sub: ChannelSubscriberRef, _min: Channel, _max: Channel);
    // Removes all channel and range subscriptions from the subscriber.
    fn unsubscribe_all(&mut self, _sub: ChannelSubscriberRef);
    // Removes the given subscriber from the multi-map for a given channel.
    // Returns true only if:
    // a) There are subscribers for the given channel and
    // b) The provided subscriber was the last one for the channel, and was removed successfully.
    fn remove_subscriber(&mut self, sub: ChannelSubscriberRef, chan: Channel) -> bool;
    // Checks if a given object has a subscription on a channel.
    fn is_subscribed(&mut self, sub: ChannelSubscriberRef, chan: Channel) -> bool;
    // Performs the same check as is_subscribed(), but for an array of channels.
    fn are_subscribed(&mut self, _subs: &mut Vec<ChannelSubscriber>, _chans: &[Channel]);
}

impl ChannelMapInterface for ChannelMap {
    fn new() -> Self {
        ChannelMap {
            subscriptions: MultiMap::new(),
            _range_subscriptions: MultiMap::new(),
        }
    }

    fn subscribe_channel(&mut self, sub: ChannelSubscriberRef, chan: Channel) {
        let mut locked_sub: MutexGuard<'_, ChannelSubscriber> = sub.lock().unwrap();
        if self.is_subscribed(sub.clone(), chan) {
            return;
        }
        locked_sub.subscribed_channels.push(chan);
        let has_subscriptions: bool = !locked_sub.subscribed_channels.is_empty();

        if has_subscriptions {
            // FIXME: Implement 'on_add_channel' callback.
        }
        self.subscriptions.insert(chan, sub.clone());
    }

    fn unsubscribe_channel(&mut self, sub: ChannelSubscriberRef, chan: Channel) {
        let mut locked_sub: MutexGuard<'_, ChannelSubscriber> = sub.lock().unwrap();
        if self.is_subscribed(sub.clone(), chan) {
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

    fn subscribe_range(&mut self, _sub: ChannelSubscriberRef, _min: Channel, _max: Channel) {}

    fn unsubscribe_range(&mut self, _sub: ChannelSubscriberRef, _min: Channel, _max: Channel) {}

    fn unsubscribe_all(&mut self, _sub: ChannelSubscriberRef) {}

    fn remove_subscriber(&mut self, sub: ChannelSubscriberRef, chan: Channel) -> bool {
        let locked_sub: MutexGuard<'_, ChannelSubscriber> = sub.lock().unwrap();
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
            for subscription in subscriptions.iter() {
                if *subscription.lock().unwrap() == *locked_sub {
                    found = true;
                    sub_count -= 1;
                }
                index += 1;
            }
            if found {
                // Okay so the remove method for values requires a second borrow of
                // the vector which the compiler didn't like, so I had to work around
                // this by using a found flag and performing the remove outside of the
                // for loop which turns values into an iterator, this way we don't
                // perform a second borrow. I don't know why it needs to be like this.
                subscriptions.swap_remove(index);
            }
        }
        sub_count == 0
    }

    fn is_subscribed(&mut self, sub: ChannelSubscriberRef, chan: Channel) -> bool {
        let locked_sub: MutexGuard<'_, ChannelSubscriber> = sub.lock().unwrap();
        if locked_sub.subscribed_channels.contains(&chan) {
            return true;
        }
        if locked_sub.subscribed_ranges.contains(&chan) {
            return true;
        }
        false
    }

    fn are_subscribed(&mut self, _subs: &mut Vec<ChannelSubscriber>, _chans: &[Channel]) {}
}
