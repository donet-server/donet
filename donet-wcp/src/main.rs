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

mod l10n;

use iced::theme::{Custom, Palette};
use iced::widget::{button, column, container, row, text, Column, Container};
use iced::window::icon::from_rgba;
use iced::window::Settings;
use iced::Length::Fill;
use iced::{color, Font, Theme};
use l10n::Localization;
use std::sync::Arc;

const DEFAULT_WIN_SIZE: (u32, u32) = (1000, 750);

/// Window icon is stored as RGBA values in the compiled binary.
static WCP_ICON: &[u8] = include_bytes!("../../logo/donet_wcp_icon.rgba");
const ICON_PX: u32 = 64;

/// Our custom theme for Donet WCP.
static WCP_PALETTE: Palette = Palette {
    background: color!(0x101010),
    text: color!(0xffffff),
    primary: color!(0xf7717d),
    success: color!(0x00ff00),
    warning: color!(0xff0900),
    danger: color!(0xff0000),
};

struct State {
    locale: Localization,
    view: TopLevelView,
}

#[derive(Default, Debug, Clone, Copy)]
enum TopLevelView {
    ControlPanel,
    #[default]
    ConnectionWizard,
}

#[derive(Debug, Clone, Copy)]
enum ToolbarCategory {
    Connection,
    Edit,
    View,
    Help,
}

impl State {
    fn new() -> State {
        let locale = Localization::default();
        State {
            locale: locale,
            view: TopLevelView::default(),
        }
    }

    fn update(&mut self, message: ToolbarCategory) {
        ()
    }

    fn view(&self) -> Column<'_, ToolbarCategory> {
        let view: Container<'_, ToolbarCategory> = match &self.view {
            TopLevelView::ControlPanel => todo!(),
            TopLevelView::ConnectionWizard => self.build_connection_wizard(),
        };

        column![self.build_toolbar(), view, self.build_status_bar()]
    }

    fn build_toolbar(&self) -> Container<'_, ToolbarCategory> {
        container(row![
            button(text(self.locale.get_string("connection"))).on_press(ToolbarCategory::Connection),
            button(text(self.locale.get_string("edit"))).on_press(ToolbarCategory::Edit),
            button(text(self.locale.get_string("view"))).on_press(ToolbarCategory::View),
            button(text(self.locale.get_string("help"))).on_press(ToolbarCategory::Help),
        ])
        .width(Fill)
        .align_left(Fill)
    }

    fn build_status_bar(&self) -> Container<'_, ToolbarCategory> {
        container(text(self.locale.get_string("status-disconnected")))
            .width(Fill)
            .align_left(Fill)
    }

    fn build_connection_wizard(&self) -> Container<'_, ToolbarCategory> {
        container(text(self.locale.get_string("connection-wizard-title")))
            .width(Fill)
            .height(Fill)
            .center(Fill)
    }

    /// Gets the localized string for the window title from our loaded Fluent bundle.
    fn title(state: &State) -> String {
        state.locale.get_string("win-title")
    }
}

fn main() -> iced::Result {
    // Force X11 on Linux + Wayland systems due to winit not fully supporting Wayland.
    //
    // Remove if the following docstring reports Wayland is now supported:
    // https://docs.rs/winit/latest/winit/window/struct.Window.html#method.set_window_icon
    //
    if let Ok(_) = std::env::var("WAYLAND_DISPLAY") {
        std::env::set_var("WAYLAND_DISPLAY", "");
    }
    let custom_palette: Arc<Custom> = Arc::new(Custom::new("".to_owned(), WCP_PALETTE));

    iced::application(State::new, State::update, State::view)
        .theme(Theme::Custom(custom_palette))
        .title(State::title)
        .window(Settings {
            size: DEFAULT_WIN_SIZE.into(),
            icon: Some(from_rgba(WCP_ICON.to_vec(), ICON_PX, ICON_PX).expect("Failed to load icon.")),
            ..Settings::default()
        })
        .default_font(Font::MONOSPACE)
        .run()
}
