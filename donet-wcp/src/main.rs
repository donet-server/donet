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
mod style;
mod wizard;

use iced::theme::Custom;
use iced::widget::{button, column, container, row, text, Column, Container};
use iced::window::icon::from_rgba;
use iced::window::Settings;
use iced::Length::Fill;
use iced::{Task, Theme};
use iced_moving_picture::widget::gif;
use l10n::Localization;
use std::sync::Arc;

use crate::wizard::ConnectionWizard;

struct State {
    locale: Localization,
    view: TopLevelView,
    frames: gif::Frames,
    wizard: wizard::ConnectionWizard,
}

#[derive(Debug, Clone)]
enum Message {
    E(wizard::WizardMessage),
    TopLevelView(TopLevelView),
    ToolbarSelection(ToolbarCategory),
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
        // pass a clone of the rc pointer for the fluent bundle to any other components
        let locale: Localization = Localization::default();

        // load the tiny gif synchronously to make sure its always loaded
        let gif_load_resp: Result<gif::Frames, gif::Error> =
            gif::Frames::from_bytes(style::LOADING_GIF_DATA.to_vec());

        State {
            locale: locale.clone(),
            view: TopLevelView::default(),
            frames: match gif_load_resp {
                Ok(frames) => frames,
                Err(err) => panic!("Error while loading gif: {err}"),
            },
            wizard: ConnectionWizard::new(locale),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::E(m) => self.wizard.update(m),
            _ => Task::none(),
        }
    }

    fn view(&self) -> Column<'_, Message> {
        let view: Container<'_, Message> = match &self.view {
            TopLevelView::ControlPanel => todo!(),
            TopLevelView::ConnectionWizard => self.wizard.view(),
        };

        column![self.build_toolbar(), view, self.build_status_bar()]
    }

    fn build_toolbar(&self) -> Container<'_, Message> {
        container(row![
            button(text(self.locale.get_string("connection")))
                .style(button::background)
                .on_press(Message::ToolbarSelection(ToolbarCategory::Connection)),
            button(text(self.locale.get_string("edit")))
                .style(button::background)
                .on_press(Message::ToolbarSelection(ToolbarCategory::Edit)),
            button(text(self.locale.get_string("view")))
                .style(button::background)
                .on_press(Message::ToolbarSelection(ToolbarCategory::Help)),
            button(text(self.locale.get_string("help")))
                .style(button::background)
                .on_press(Message::ToolbarSelection(ToolbarCategory::View)),
        ])
        .width(Fill)
        .align_left(Fill)
    }

    fn build_status_bar(&self) -> Container<'_, Message> {
        container(
            container(row![
                text("m ").font(style::ICONS_FONT).size(20),
                container(text(self.locale.get_string("status-disconnected"))).padding(3)
            ])
            .align_left(Fill),
        )
        .width(Fill)
        .padding(7)
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
    let custom_palette: Arc<Custom> = Arc::new(Custom::new("".to_owned(), style::WCP_PALETTE));

    iced::application(State::new, State::update, State::view)
        .theme(Theme::Custom(custom_palette))
        .title(State::title)
        .window(Settings {
            size: style::DEFAULT_WIN_SIZE.into(),
            icon: Some(
                from_rgba(style::APP_ICON_DATA.to_vec(), style::ICON_PX, style::ICON_PX)
                    .expect("Failed to load icon."),
            ),
            min_size: Some(style::MIN_WIN_SIZE.into()),
            ..Settings::default()
        })
        .font(style::CANTARELL_FONT_DATA)
        .font(style::ICONS_FONT_DATA)
        .default_font(style::DEFAULT_FONT)
        .run()
}
