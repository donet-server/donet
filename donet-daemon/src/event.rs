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

use donet_core::datagram::datagram::Datagram;

/// This structure provides a generic and easy to use interface to build
/// new events to be sent to the Event Logger in the [`MessagePack`] format.
///
/// [`MessagePack`]: https://msgpack.org
pub struct LoggedEvent {
    elements: Vec<(String, String)>,
}

impl LoggedEvent {
    pub fn new(event_type: &str, sender: &str) -> Self {
        Self {
            elements: vec![
                ("type".to_string(), event_type.to_string()),
                ("sender".to_string(), sender.to_string()),
            ],
        }
    }

    pub fn add(&mut self, key: &str, value: &str) {
        self.elements.push((key.to_string(), value.to_string()));
    }

    pub fn make_datagram(&self) -> Datagram {
        let mut dg = Datagram::default();
        let map_len: usize = self.elements.len();

        if map_len < 16 {
            // Can fit in a fixmap.
            dg.add_u8((0x80 + map_len) as u8).unwrap();
        } else {
            // Use a map16.
            // We don't have to worry about map32, nothing that big will fit in a
            // single UDP packet anyway.
            dg.add_u8(0xde).unwrap();
            dg.add_u8((map_len >> 8 & 0xff) as u8).unwrap();
            dg.add_u8((map_len & 0xff) as u8).unwrap();
        }

        for (key, value) in self.elements.iter() {
            Self::pack_string(&mut dg, key);
            Self::pack_string(&mut dg, value);
        }
        dg
    }

    #[inline(always)]
    fn pack_string(dg: &mut Datagram, value: &str) {
        let size: usize = value.len();

        if size < 32 {
            // Small enough for a msgpack fixstr.
            dg.add_u8((0xa0 + size) as u8).unwrap();
        } else {
            // Use a str16.
            // We don't have to worry about str32, nothing that big will fit in a
            // single UDP packet anyway.
            dg.add_u8(0xda).unwrap();
            dg.add_u8((size >> 8 & 0xff) as u8).unwrap();
            dg.add_u8((size & 0xff) as u8).unwrap();
        }

        dg.add_data(value.as_bytes().to_vec()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::{Datagram, LoggedEvent};

    #[test]
    fn fixmap_fixstr_msgpack_datagram() {
        let mut event: LoggedEvent = LoggedEvent::new("unit", "test");
        event.add("msg", "Unit Test Event");

        let _: Datagram = event.make_datagram();
    }

    #[test]
    fn map16_str16_msgpack_datagram() {
        let mut event: LoggedEvent = LoggedEvent::new("unit", "test");

        // Add more than 2 ^ 4 - 1, or 15, elements to the map.
        for _ in 0..16 {
            event.add("msg", "Unit Test Event");
        }

        // Add a string that is larger than 2 ^ 5 - 1, or 32, chars.
        event.add("msg", "0123456789abcdefghijklmnopqrstuvwxyz");

        let _: Datagram = event.make_datagram();
    }
}
