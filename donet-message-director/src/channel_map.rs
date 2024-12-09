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

use super::subscriber::*;
use donet_core::globals::Channel;
use gcollections::ops::*;
use interval::interval_set::ToIntervalSet;
use interval::IntervalSet;
use multimap::MultiMap;
use rangemap::RangeInclusiveMap;
use std::collections::HashSet;
use std::ops::{Range, RangeInclusive};
use tokio::sync::MutexGuard;

/// Iterates over all ranges in a [`rangemap::RangeInclusiveMap`],
/// and filters out ranges that do NOT overlap with the given
/// `target` range.
fn equal_range(
    map: &RangeInclusiveMap<Channel, HashSet<SubscriberRef>>,
    target: Range<Channel>,
) -> Vec<(&RangeInclusive<Channel>, &HashSet<SubscriberRef>)> {
    map.iter()
        .filter(|(range, _)| {
            // Check if the range overlaps with the target range
            range.start() < &target.end && range.end() > &target.start
        })
        .collect()
}

/// Data model to store all channel subscriptions created via a
/// Message Director service instance.
///
/// Functionality with callbacks for handling changes in mapping can
/// be achieved by implementing the [`ChannelCoordinator`] trait.
#[derive(Default)]
pub struct ChannelMap {
    /// Single channel subscriptions
    subscriptions: MultiMap<Channel, SubscriberRef>,
    /// Channel range subscriptions
    range_subscriptions: RangeInclusiveMap<Channel, HashSet<SubscriberRef>>,
}

/// Struct implementing this trait must own a [`ChannelMap`].
pub trait HasChannelMap {
    fn get_channel_map(&mut self) -> &mut ChannelMap;
}

/// Provides functionality for mapping channels, as [`Channel`],
/// to objects interested in that channel, as [`ChannelSubscriber`],
/// using a [`ChannelMap`] structure stored by the implementer.
///
/// The implementing type must also implement [`HasChannelMap`],
/// to guarantee that there is a [`ChannelMap`] in memory.
pub trait ChannelCoordinator
where
    Self: HasChannelMap,
{
    // Callbacks that must be implemented manually.
    async fn on_add_channel(&mut self, channel: Channel);
    async fn on_remove_channel(&mut self, channel: Channel);
    async fn on_add_range(&mut self, range: Range<Channel>);
    async fn on_remove_range(&mut self, range: Range<Channel>);

    /// Adds a single channel to the subscriber's subscribed channels map.
    async fn subscribe_channel(&mut self, sub: SubscriberRef, chan: Channel) {
        let mut locked_sub: MutexGuard<'_, Subscriber> = sub.lock().await;

        if Self::is_subscribed(self, &locked_sub, chan).await {
            return;
        }

        locked_sub.subscribed_channels.insert(chan);

        let has_subscriptions: bool = !locked_sub.subscribed_channels.is_empty();

        if has_subscriptions {
            Self::on_add_channel(self, chan).await;
        }
        self.get_channel_map().subscriptions.insert(chan, sub.clone());
    }

    /// Removes the given channel from the subscribed channels map.
    ///
    /// Does nothing if the subscriber is not subscribed to the given
    /// channel.
    async fn unsubscribe_channel(&mut self, sub: SubscriberRef, chan: Channel) {
        let mut locked_sub: MutexGuard<'_, Subscriber> = sub.lock().await;

        if !Self::is_subscribed(self, &locked_sub, chan).await {
            return;
        }

        locked_sub.subscribed_channels.remove(&chan);

        // release mutex to allow the remove sub function to lock
        drop(locked_sub);

        if Self::remove_subscriber(self, sub.clone(), chan).await {
            Self::on_remove_channel(self, chan).await;
        }
    }

    /// Adds an object to be subscribed to a range of channels.
    ///
    /// The given range is inclusive.
    async fn subscribe_range(&mut self, sub: SubscriberRef, min: Channel, max: Channel) {
        {
            let mut locked_sub: MutexGuard<'_, Subscriber> = sub.lock().await;

            // Create a new closed interval set using given range
            let new_interval: IntervalSet<Channel> = vec![(min, max)].to_interval_set();

            // Create a new set with the given subscriber
            let mut new_sub_set: HashSet<SubscriberRef> = HashSet::default();
            new_sub_set.insert(sub.clone());

            // Update channel range subscription mappings
            locked_sub.subscribed_ranges.extend(new_interval);

            self.get_channel_map()
                .range_subscriptions
                .insert(RangeInclusive::new(min, max), new_sub_set);
        }

        // Finally, check if any part of this interval is a new range.
        // (Check if any range of this interval does NOT overlap an existing range.)
        let new_range: Range<Channel> = min..max;

        // Get overlapping ranges from map's range subscription map.
        let interval_range: Vec<_> = equal_range(&self.get_channel_map().range_subscriptions, new_range);

        for segment in interval_range {
            if segment.1.len() == 1 {
                // There's a segment of the interval that has only one item
                // (our newly added subscriber), which confirms this IS a new
                // range. So, we should upstream the range addition.
                Self::on_add_range(self, min..max).await;
                break;
            }
        }
    }

    /// Performs the reverse of the `Self::subscribe_range()` function.
    ///
    /// The given range is inclusive.
    async fn unsubscribe_range(&mut self, sub: SubscriberRef, min: Channel, max: Channel) {
        // if we have no range subscriptions mapped, do nothing and return early
        if self.get_channel_map().range_subscriptions.is_empty() {
            return;
        }

        // prepare subscriber set
        let mut sub_set: HashSet<SubscriberRef> = HashSet::default();
        sub_set.insert(sub.clone());

        // Construct the interval we are removing, bounded to range subscriptions.
        let map: &mut ChannelMap = self.get_channel_map();

        // We can simply unwrap the first/last as we check that there is at least
        // one range subscription in the beginning of this function.
        let rs_first = map.range_subscriptions.first_range_value().unwrap();
        let rs_last = map.range_subscriptions.last_range_value().unwrap();

        let lower: Channel = *rs_first.0.start();
        let upper: Channel = *rs_last.0.end();

        let union_lower: Channel = std::cmp::max(min, lower);
        let union_upper: Channel = std::cmp::max(max, upper);

        let range: Range<Channel> = union_lower..union_upper;

        let i_set: IntervalSet<Channel> = vec![(union_lower, union_upper)].to_interval_set();

        // Speculate the channel ranges that will have no subscribers
        // after this subscriber is removed.
        let mut dead_ranges: IntervalSet<Channel> = i_set.clone();
        let interval_range = equal_range(&map.range_subscriptions, union_lower..union_upper);

        // go through interval range and remove ranges that will still
        // have subscribers after this subscriber is removed
        for (range, range_subs) in interval_range {
            let has_subscribers: bool = !range_subs.is_empty();
            let is_only_subscriber: bool = (range_subs.len() == 1) && range_subs.contains(&sub);

            if has_subscribers && !is_only_subscriber {
                // we are not the last subscriber in this range, so don't delete it
                dead_ranges = &dead_ranges - vec![(*range.start(), *range.end())].to_interval_set();
            }
        }

        // update range mappings on both subscriber and channel map
        let mut locked_sub: MutexGuard<'_, Subscriber> = sub.lock().await;

        locked_sub.subscribed_ranges = &locked_sub.subscribed_ranges - i_set;
        map.range_subscriptions
            .remove(RangeInclusive::new(range.start, range.end));

        // clone subscriber's channel subscriptions to avoid double borrow
        let chans = locked_sub.subscribed_channels.clone();

        // delete single channel subscriptions that fall within the range
        for channel in &chans {
            if (min <= *channel) && (*channel <= max) {
                // we do **not** call `Self::unsubscribe_channel`, because
                // that might send off an `on_remove_channel` event.
                // Instead, we should update this manually.
                Self::remove_subscriber(self, sub.clone(), *channel).await;

                locked_sub.subscribed_channels.remove(channel);
            }
        }

        // finally, have our channel coordinator delete any new 'dead' ranges
        for range in dead_ranges {
            Self::on_remove_range(self, range.lower()..range.upper()).await;
        }
    }

    /// Removes all channel and range subscriptions from the subscriber.
    async fn unsubscribe_all(&mut self, sub: SubscriberRef) {
        let locked_sub: MutexGuard<'_, Subscriber> = sub.lock().await;

        // take copies of subscriber's subscriptions to release mutex below
        let single_chan_subs = locked_sub.subscribed_channels.clone();
        let range_subs = locked_sub.subscribed_ranges.clone();

        // release mutex to allow the unsub function to lock
        drop(locked_sub);

        for channel in single_chan_subs.into_iter() {
            // call unsub function, with a clone of our pointer to the sub
            Self::unsubscribe_channel(self, sub.clone(), channel).await;
        }

        for range in range_subs.into_iter() {
            let min: Channel = range.lower();
            let max: Channel = range.upper();

            Self::unsubscribe_range(self, sub.clone(), min, max).await;
        }
    }

    /// Removes the given subscriber from the MultiMap for a given
    /// channel.
    ///
    /// Returns `true` **only** if:
    ///
    /// - There are subscribers for the given channel.
    ///
    /// - The provided subscriber was the last one for the channel,
    ///   and was removed successfully.
    ///
    async fn remove_subscriber(&mut self, sub: SubscriberRef, chan: Channel) -> bool {
        let map: &mut ChannelMap = self.get_channel_map();

        let locked_sub: MutexGuard<'_, Subscriber> = sub.lock().await;
        let mut sub_count: usize = map.subscriptions.len();

        if sub_count == 0 {
            return false;
        }
        for (key, subscriptions) in map.subscriptions.iter_all_mut() {
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

    /// Checks if a given subscriber has a subscription on the given
    /// channel.
    ///
    /// Looks over both single and range channel subscriptions.
    async fn is_subscribed(&self, sub_lock: &MutexGuard<'_, Subscriber>, chan: Channel) -> bool {
        if sub_lock.subscribed_channels.contains(&chan) {
            return true;
        }
        if sub_lock.subscribed_ranges.contains(&chan) {
            return true;
        }
        false
    }

    /// Populates a set with the subscribers for a list of channels.
    fn lookup_channels(&mut self, channels: Vec<Channel>, subs: &mut HashSet<SubscriberRef>) {
        for channel in channels {
            // Run through single-channel subscriptions map
            if let Some(chan_subs) = self.get_channel_map().subscriptions.get_vec(&channel) {
                subs.extend(chan_subs.iter().cloned());
            }

            // Run through range subscriptions map
            for (_range, range_subs) in self
                .get_channel_map()
                .range_subscriptions
                .overlapping(RangeInclusive::new(channel, channel))
            {
                subs.extend(range_subs.iter().cloned());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::str::FromStr;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[derive(Default)]
    struct MockChannelCoordinator {
        map: ChannelMap,
        // Flags for if an 'on_xxx' callback was triggered.
        got_add_channel: AtomicBool,
        got_add_range: AtomicBool,
        got_remove_channel: AtomicBool,
        got_remove_range: AtomicBool,
    }

    impl HasChannelMap for MockChannelCoordinator {
        fn get_channel_map(&mut self) -> &mut ChannelMap {
            &mut self.map
        }
    }

    impl ChannelCoordinator for MockChannelCoordinator {
        async fn on_add_channel(&mut self, _channel: Channel) {
            self.got_add_channel.swap(true, Ordering::SeqCst);
        }

        async fn on_add_range(&mut self, _range: Range<Channel>) {
            self.got_add_range.swap(true, Ordering::SeqCst);
        }

        async fn on_remove_channel(&mut self, _channel: Channel) {
            self.got_remove_channel.swap(true, Ordering::SeqCst);
        }

        async fn on_remove_range(&mut self, _range: Range<Channel>) {
            self.got_remove_range.swap(true, Ordering::SeqCst);
        }
    }

    #[tokio::test]
    async fn single_subscription() {
        let mut mock = MockChannelCoordinator::default();
        let mock_sub_1 = SubscriberRef::from(SocketAddr::from_str("127.0.0.1:1").unwrap());

        mock.subscribe_channel(mock_sub_1.clone(), 1000).await;

        // verify that the `on_add_channel` callback was triggered
        assert!(*mock.got_add_channel.get_mut());

        assert!(mock.is_subscribed(&mock_sub_1.lock().await, 1000).await);
    }

    #[tokio::test]
    async fn range_subscription() {
        let mut mock = MockChannelCoordinator::default();
        let mock_sub_1 = SubscriberRef::from(SocketAddr::from_str("127.0.0.1:1").unwrap());

        // test range subscription
        let min: Channel = 1000;
        let max: Channel = 2000;

        mock.subscribe_range(mock_sub_1.clone(), min, max).await;

        let sub_lock = mock_sub_1.lock().await;
        eprintln!("{:#?}", sub_lock);

        // verify that the `on_add_range` callback was triggered
        assert!(*mock.got_add_range.get_mut());

        for i in min..max {
            eprintln!("{}", i);
            assert!(mock.is_subscribed(&sub_lock, i).await);
        }

        assert!(!mock.is_subscribed(&sub_lock, min - 1).await);
        assert!(!mock.is_subscribed(&sub_lock, max + 1).await);
    }
}
