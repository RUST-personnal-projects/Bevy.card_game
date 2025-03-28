pub mod asset_loading;

use asset_loading::TestAssetLoadingState;
use bevy::{prelude::*, state::app::StatesPlugin};

pub fn plugin(app: &mut App) {
    // During tests, StatesPlugin might not have been added
    if !app.is_plugin_added::<StatesPlugin>() {
        app.add_plugins(StatesPlugin);
    }
    app.add_plugins((AssetPlugin::default(), ImagePlugin::default()))
        .init_state::<TestAssetLoadingState>();
}
