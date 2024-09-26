//! The screen state for the main game loop.
pub(crate) mod game_loop;
pub(crate) mod setup;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((setup::plugin, game_loop::plugin));
}
