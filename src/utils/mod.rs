pub(crate) mod assets;
pub(crate) mod mouse;
//mod dev_tools;

#[cfg(test)]
pub(crate) mod test;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        mouse::plugin,
        assets::plugin,
        //dev_tools::plugin,
    ));
}
