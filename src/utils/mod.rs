mod assets;
mod mouse;
//mod dev_tools;

#[cfg(test)]
pub mod test;

use bevy::prelude::*;

pub(super) mod mousePrelude {
    pub use super::mouse::MouseInteractionBundle;
    pub use super::{
        mouse::{Clickable, Clicked},
        mouse::{Hoverable, Hovered},
        mouse::{MouseCoordinates, UIMouseCoordinates},
    };
}

pub(super) use assets::{is_asset_loaded, Loaded};

#[cfg(test)]
pub(super) use test::{count_entities, EntityCount};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        mouse::plugin,
        //dev_tools::plugin,
    ));
}
