pub mod assets;
pub mod image_scaling;
pub mod mouse;
pub mod ui;

#[cfg(test)]
pub mod test;

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
pub struct UtilsBundle {
    mouse: MouseInteractionBundle,
    scaled_size: ScaledSize,
}
