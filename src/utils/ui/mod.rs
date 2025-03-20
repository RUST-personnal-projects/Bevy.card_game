//! Reusable UI widgets & theming.

// Unused utilities and re-exports may trigger these lints undesirably.
#![allow(dead_code, unused_imports)]

pub mod interaction;
mod widgets;

use bevy::prelude::*;

pub mod prelude {
    pub use super::{
        interaction::{InteractionPalette, InteractionQuery},
        widgets::{Containers as _, Widgets as _},
    };
}

pub mod palette {
    use super::*;

    pub const BUTTON_HOVERED_BACKGROUND: Color = Color::srgb(0.186, 0.328, 0.573);
    pub const BUTTON_PRESSED_BACKGROUND: Color = Color::srgb(0.286, 0.478, 0.773);

    pub const BUTTON_TEXT: Color = Color::srgb(0.925, 0.925, 0.925);
    pub const LABEL_TEXT: Color = Color::srgb(0.867, 0.827, 0.412);
    pub const HEADER_TEXT: Color = Color::srgb(0., 0., 0.);

    pub const NODE_BACKGROUND: Color = Color::srgb(0.286, 0.478, 0.773);

    pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
    pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
    pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
}
