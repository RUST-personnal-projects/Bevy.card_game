pub(crate) mod card;
pub(crate) mod deck;
pub(crate) mod screen;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((deck::plugin, screen::plugin));
}
