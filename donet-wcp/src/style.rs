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

use iced::color;
use iced::font::{Family, Font, Stretch, Style, Weight};
use iced::theme::Palette;

pub const DEFAULT_WIN_SIZE: (u32, u32) = (1000, 750);
pub const MIN_WIN_SIZE: (u32, u32) = (500, 500);

/// Window icon is stored as RGBA values in the compiled binary.
pub static APP_ICON_DATA: &[u8] = include_bytes!("../assets/icon.rgba");
pub const ICON_PX: u32 = 32;

pub static LOADING_GIF_DATA: &[u8] = include_bytes!("../assets/loading.gif");

/// Our custom theme for Donet WCP.
pub static WCP_PALETTE: Palette = Palette {
    background: color!(0x151515),
    text: color!(0xffffff),
    primary: color!(0xf7717d),
    success: color!(0x00ff00),
    warning: color!(0xff0900),
    danger: color!(0xff0000),
};

// Store font files in the compiled binary.
pub static CANTARELL_FONT_DATA: &[u8] = include_bytes!("../assets/Cantarell-VF.otf");
pub static ICONS_FONT_DATA: &[u8] = include_bytes!("../assets/icons.otf");

/// Iced [`Font`] struct to query our default font.
pub static DEFAULT_FONT: Font = Font {
    family: Family::Name("Cantarell"),
    weight: Weight::Normal,
    stretch: Stretch::Normal,
    style: Style::Normal,
};

/// Iced [`Font`] struct to query our Icons font.
pub static ICONS_FONT: Font = Font {
    family: Family::Name("wcpicons"),
    weight: Weight::Medium,
    stretch: Stretch::Normal,
    style: Style::Normal,
};

pub struct Typography {
    pub title: f32,
    pub header: f32,
    pub body: f32,
    pub small: f32,
}

pub static TYPOGRAPHY: Typography = Typography {
    title: 30.0,
    header: 20.0,
    body: 15.0,
    small: 8.0,
};
