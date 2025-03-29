//! The screen state for loading game assets.

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::utils::{assets::CardImageAssets, ui::prelude::*};

use super::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), enter_loading)
        .add_loading_state(
            LoadingState::new(Screen::Loading)
                .continue_to_state(Screen::Playing)
                .load_collection::<CardImageAssets>(),
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
