use bevy::prelude::*;

pub mod loader;

use loader::is_asset_loaded;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, is_asset_loaded::<Image>);
    }
}
