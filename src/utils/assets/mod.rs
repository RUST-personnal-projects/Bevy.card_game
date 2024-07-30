mod loader;

use bevy::prelude::*;

pub use loader::{is_asset_loaded, Loaded};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, is_asset_loaded::<Image>);
}
