pub mod cards;
pub mod fixed_entities;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((cards::plugin, fixed_entities::plugin));
}
