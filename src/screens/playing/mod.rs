//! The screen state for the main game loop.
pub mod game_loop;
pub mod setup;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((setup::plugin, game_loop::plugin));
}
