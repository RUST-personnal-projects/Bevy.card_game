pub(crate) mod assets;
#[cfg(feature = "dev")]
pub(crate) mod dev_tools;
pub(crate) mod mouse;

#[cfg(test)]
pub(crate) mod test;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((mouse::plugin, assets::plugin));
    #[cfg(feature = "dev")]
    app.add_plugins(dev_tools::plugin);
}
