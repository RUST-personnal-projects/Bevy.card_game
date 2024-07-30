mod asset_loading;
mod count_entities;

use bevy::{prelude::*, render::texture::ImageLoader, state::app::StatesPlugin};

pub(super) use self::asset_loading::{check_loaded, TestAssetLoadingState};
pub use self::count_entities::{count_entities, EntityCount};

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
