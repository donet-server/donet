// DONET SOFTWARE
// Copyright (c) 2023, DoNet Authors.
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

use crate::globals::Channel;
use multimap::MultiMap;
use std::vec::Vec;

#[derive(Clone)]
pub struct ChannelSubscriber {
    pub subscribed_channels: Vec<Channel>,
    pub subscribed_ranges: Vec<Channel>,
}

pub struct ChannelMap {
    // FIXME: Astron stores pointers to ChannelSubscribers (equivalent
    // to unsafe raw pointers in Rust), but here we store clones of
    // these ChannelSubscriber structs. Not sure if we'll need to change
    // to another method where transfer of ownership is required or something.
    subscriptions: MultiMap<Channel, ChannelSubscriber>,
    range_subscriptions: MultiMap<Channel, Vec<ChannelSubscriber>>,
}

trait ChannelMapInterface {
    fn new() -> ChannelMap;
    fn subscribe_channel(&mut self, sub: &mut ChannelSubscriber, chan: Channel) -> ();
    fn unsubscribe_channel(&mut self, sub: &mut ChannelSubscriber, chan: Channel) -> ();
    fn subscribe_range(&mut self, sub: &mut ChannelSubscriber, min: Channel, max: Channel) -> ();
    fn unsubscribe_range(&mut self, sub: &mut ChannelSubscriber, min: Channel, max: Channel) -> ();
    fn unsubscribe_all(&mut self, sub: &mut ChannelSubscriber) -> ();
    fn remove_subscriber(&mut self, sub: &mut ChannelSubscriber, chan: Channel) -> bool;
    fn is_subscribed(&mut self, sub: &mut ChannelSubscriber, chan: Channel) -> bool;
    fn are_subscribed(&mut self, subs: &mut Vec<ChannelSubscriber>, chans: &Vec<Channel>) -> ();
}

impl ChannelMapInterface for ChannelMap {
    fn new() -> ChannelMap {
        return ChannelMap {
            subscriptions: MultiMap::new(),
            range_subscriptions: MultiMap::new(),
        };
    }
    fn subscribe_channel(&mut self, sub: &mut ChannelSubscriber, chan: Channel) -> () {
        if self.is_subscribed(sub, chan) {
            return;
        }
        sub.subscribed_channels.push(chan);
        let has_subscriptions: bool = sub.subscribed_channels.len() != 0;

        if has_subscriptions {
            // FIXME: Implement 'on_add_channel' callback.
        }
        self.subscriptions.insert(chan, sub.clone());
    }
    fn unsubscribe_channel(&mut self, sub: &mut ChannelSubscriber, chan: Channel) -> () {}
    fn subscribe_range(&mut self, sub: &mut ChannelSubscriber, min: Channel, max: Channel) -> () {}
    fn unsubscribe_range(&mut self, sub: &mut ChannelSubscriber, min: Channel, max: Channel) -> () {}
    fn unsubscribe_all(&mut self, sub: &mut ChannelSubscriber) -> () {}
    fn remove_subscriber(&mut self, sub: &mut ChannelSubscriber, chan: Channel) -> bool {
        return false;
    }
    fn is_subscribed(&mut self, sub: &mut ChannelSubscriber, chan: Channel) -> bool {
        return false;
    }
    fn are_subscribed(&mut self, subs: &mut Vec<ChannelSubscriber>, chans: &Vec<Channel>) -> () {}
}
