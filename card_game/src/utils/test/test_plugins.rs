use bevy::{prelude::*, render::texture::ImageLoader};

use super::asset_loading::TestAssetLoadingState;

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AssetPlugin::default())
            .init_asset::<Image>()
            .init_asset_loader::<ImageLoader>()
            .init_state::<TestAssetLoadingState>()
            .init_resource::<Assets<Image>>();
    }
}
