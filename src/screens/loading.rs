//! The screen state for loading game assets.

use bevy::prelude::*;

use crate::utils::{
    assets::{images::ImageKey, HandleMap},
    ui::prelude::*,
};

use super::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), enter_loading);

    app.add_systems(
        Update,
        go_to_playing_screen.run_if(in_state(Screen::Loading).and_then(all_assets_loaded)),
    );
}

fn enter_loading(mut commands: Commands) {
    commands
        .ui_root(None, None)
        .insert(StateScoped(Screen::Loading))
        .with_children(|children| {
            children.label("Loading...");
        });
}

fn all_assets_loaded(
    asset_server: Res<AssetServer>,
    image_handles: Res<HandleMap<ImageKey>>,
) -> bool {
    image_handles.all_loaded(&asset_server)
}

fn go_to_playing_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Playing);
}
