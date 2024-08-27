pub(crate) mod assets;
pub(crate) mod mouse;
pub(crate) mod ui;

#[cfg(test)]
pub(crate) mod test;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((mouse::plugin, assets::plugin, ui::plugin));
}
