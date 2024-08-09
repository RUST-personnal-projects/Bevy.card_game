use bevy::{asset::AssetMetaCheck, prelude::*};
use game::card::{Card, CardColor, ColoredVariant, WildVariant};
use utils::mouse::MouseInteractionBundle;

mod game;
mod utils;

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
                })
                .set(ImagePlugin::default_nearest()),
        );

        // Project Plugins
        app.add_plugins((utils::plugin, game::plugin));

        // TODO: remove this setup once scene are loaded automatically
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    for (card, transform) in [
        (
            Card::Colored(ColoredVariant::Number(9), CardColor::Blue),
            Transform::from_xyz(-300., 0., 0.),
        ),
        (
            Card::Wild(WildVariant::ColorChange),
            Transform::from_xyz(-100., 0., 0.),
        ),
        (
            Card::Wild(WildVariant::PlusFour),
            Transform::from_xyz(100., 0., 0.),
        ),
        (
            Card::Colored(ColoredVariant::Invert, CardColor::Yellow),
            Transform::from_xyz(300., 0., 0.),
        ),
    ] {
        let texture = asset_server.load(card.texture_path());

        commands.spawn((
            card,
            SpriteBundle {
                texture,
                transform,
                ..default()
            },
            MouseInteractionBundle::default(),
        ));
    }
}
