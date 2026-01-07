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

use crate::l10n::Localization;
use crate::style::TYPOGRAPHY;

use std::path::PathBuf;

use iced::widget::{button, column, container, row, text, text_input, Container, Space};
use iced::Length::{Fill, FillPortion};
use iced::{Font, Task};

#[derive(Debug, Clone)]
pub enum WizardMessage {
    ContentChanged,
}

pub struct ConnectionWizard {
    locale: Localization,
    md_address_textbox: String,
    md_port_textbox: String,
    dc_hash_override_textbox: String,
    dc_files: Vec<PathBuf>,
}

impl ConnectionWizard {
    pub fn new(locale: Localization) -> ConnectionWizard {
        ConnectionWizard {
            locale,
            md_address_textbox: String::default(),
            md_port_textbox: String::default(),
            dc_hash_override_textbox: String::default(),
            dc_files: vec![],
        }
    }

    pub fn update(&self, message: WizardMessage) -> Task<crate::Message> {
        Task::none()
    }

    pub fn view(&self) -> Container<'_, crate::Message> {
        container(
            container(
                column![
                    container(
                        text(self.locale.get_string("connection-wizard-title"))
                            .font(Font {
                                family: iced::font::Family::Name("Cantarell"),
                                weight: iced::font::Weight::ExtraBold,
                                ..Font::DEFAULT
                            })
                            .color(iced::color!(0xfff2f3))
                            .size(TYPOGRAPHY.title)
                    )
                    .width(Fill)
                    .height(Fill)
                    .center(Fill),
                    row![
                        column![
                            text(self.locale.get_string("md-address-field"))
                                .height(FillPortion(1))
                                .font(Font {
                                    weight: iced::font::Weight::Light,
                                    ..Font::DEFAULT
                                })
                                .size(crate::style::TYPOGRAPHY.body),
                            text(self.locale.get_string("md-port-field"))
                                .height(FillPortion(1))
                                .font(Font {
                                    weight: iced::font::Weight::Light,
                                    ..Font::DEFAULT
                                })
                                .size(crate::style::TYPOGRAPHY.body),
                            text(self.locale.get_string("manual-dc-hash"))
                                .height(FillPortion(1))
                                .font(Font {
                                    weight: iced::font::Weight::Light,
                                    ..Font::DEFAULT
                                })
                                .size(crate::style::TYPOGRAPHY.body),
                            text(self.locale.get_string("dc-files"))
                                .height(FillPortion(1))
                                .font(Font {
                                    weight: iced::font::Weight::Light,
                                    ..Font::DEFAULT
                                })
                                .size(crate::style::TYPOGRAPHY.body),
                        ]
                        .width(FillPortion(5)),
                        iced::widget::Space::new().width(FillPortion(1)),
                        column![
                            text_input("127.0.0.1", &self.md_address_textbox),
                            text_input("7199", &self.md_port_textbox),
                            text_input("0xabcdef", &self.dc_hash_override_textbox),
                            button(text(self.locale.get_string("choose-dc-file"))),
                        ]
                        .width(FillPortion(4)),
                    ],
                    container(button(text(self.locale.get_string("connect"))))
                        .width(Fill)
                        .height(Fill)
                        .center(Fill),
                ]
                .width(Fill)
                .height(Fill),
            )
            .max_height(400)
            .max_width(400),
        )
        .center(Fill)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
        .padding(50)
    }
}
