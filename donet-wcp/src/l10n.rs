/*
    This file is part of Donet.

    Copyright © 2026 Max Rodriguez <me@maxrdz.com>

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

use fluent::{FluentArgs, FluentBundle, FluentError, FluentMessage, FluentResource};
use std::rc::Rc;
use unic_langid::LanguageIdentifier;

#[derive(Clone)]
pub struct Localization {
    inner: Rc<FluentBundle<FluentResource>>,
}

impl Default for Localization {
    fn default() -> Self {
        let ftl_read: Vec<u8> = std::fs::read("./donet-wcp/l10n/en-US/app.ftl").unwrap();
        let ftl_string: String = String::from_utf8(ftl_read).unwrap();

        let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");

        let langid_en: LanguageIdentifier = "en-US".parse().expect("Parsing failed");
        let mut bundle = FluentBundle::new(vec![langid_en]);

        bundle
            .add_resource(res)
            .expect("Failed to add FTL resources to the bundle.");

        Self {
            inner: Rc::new(bundle),
        }
    }
}

impl Localization {
    /// Retrieves a Fluent message from the Fluent bundle.
    ///
    /// ## Panics
    /// - Expects the given ID to be valid.
    /// - Expects all Fluent messages to have values.
    ///
    pub fn get_string_fmt(&self, id: &str, fmt_args: Option<&FluentArgs<'_>>) -> String {
        let mut errors: Vec<FluentError> = vec![];

        let e: FluentMessage<'_> = self
            .inner
            .get_message(id)
            .expect("Could not find Fluent message!");
        let f = e.value().expect("Fluent message has no value!");

        self.inner.format_pattern(f, fmt_args, &mut errors).to_string()
    }

    /// Wrapper of [`Localization::get_string_fmt`],
    /// but passes `None` for its formatting arguments.
    ///
    pub fn get_string(&self, id: &str) -> String {
        self.get_string_fmt(id, None)
    }
}
