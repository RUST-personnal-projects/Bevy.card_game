pub(crate) mod assets;
pub(crate) mod image_scaling;
pub(crate) mod mouse;
pub(crate) mod ui;

#[cfg(test)]
pub(crate) mod test;

use bevy::prelude::*;
use image_scaling::ScaledSize;
use mouse::MouseInteractionBundle;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        mouse::plugin,
        assets::plugin,
        ui::plugin,
        image_scaling::plugin,
    ));
}

#[derive(Bundle, Debug, Default)]
pub(crate) struct UtilsBundle {
    mouse: MouseInteractionBundle,
    scaled_size: ScaledSize,
}
