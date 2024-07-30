pub(crate) mod asset_loading;
pub(crate) mod count_entities;

use asset_loading::TestAssetLoadingState;
use bevy::{prelude::*, render::texture::ImageLoader, state::app::StatesPlugin};

pub(super) fn plugin(app: &mut App) {
    // During tests, StatesPlugin might not have been added
    if !app.is_plugin_added::<StatesPlugin>() {
        app.add_plugins(StatesPlugin);
    }
    app.add_plugins(AssetPlugin::default())
        .init_asset::<Image>()
        .init_asset_loader::<ImageLoader>()
        .init_state::<TestAssetLoadingState>()
        .init_resource::<Assets<Image>>();
}
