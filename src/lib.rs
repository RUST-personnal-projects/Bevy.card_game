use bevy::{prelude::*, window::*};

mod features;
mod utils;

use features::{
    cards::{CardBundle, CardColor, CardVariant},
    deck,
};
use utils::mouse;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Bevy plugins
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "bevy_quickstart".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        );

        // Project Plugins
        app.add_plugins((mouse::MousePlugins, deck::DeckPlugin));

        // TODO: remove this setup once scene are loaded automatically
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    for (color, variant, transform) in [
        (
            CardColor::Blue,
            CardVariant::Number(9),
            Transform::from_xyz(-300., 0., 0.),
        ),
        (
            CardColor::Wild,
            CardVariant::Wild,
            Transform::from_xyz(-100., 0., 0.),
        ),
        (
            CardColor::Wild,
            CardVariant::PlusFour,
            Transform::from_xyz(100., 0., 0.),
        ),
        (
            CardColor::Yellow,
            CardVariant::Invert,
            Transform::from_xyz(300., 0., 0.),
        ),
    ] {
        let texture_path = format!("assets/{}", CardBundle::texture_path(color, variant));

        let texture = asset_server.load(texture_path);

        commands.spawn((
            CardBundle { color, variant },
            SpriteBundle {
                texture,
                transform,
                ..default()
            },
            mouse::MouseInteractionBundle::default(),
        ));
    }
}
