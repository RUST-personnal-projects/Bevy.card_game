use bevy::{asset::LoadState, prelude::*};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum TestAssetLoadingState {
    #[default]
    Loading,
    Loaded,
}

/// checks that all Assets of type T are loaded before setting state
pub fn check_loaded<T: Asset>(
    handles_query: Query<&Handle<T>>,
    assets: Res<AssetServer>,
    mut loading_state: ResMut<NextState<TestAssetLoadingState>>,
) {
    if handles_query
        .iter()
        .map(|handle| assets.load_state(handle) == LoadState::Loaded)
        .all(|is_loaded| is_loaded)
    {
        loading_state.set(TestAssetLoadingState::Loaded);
    }
}
