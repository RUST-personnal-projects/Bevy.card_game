use bevy::{prelude::*, render::texture::ImageLoader, state::app::StatesPlugin};

use super::asset_loading::TestAssetLoadingState;

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
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
}
