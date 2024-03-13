use bevy::prelude::*;

mod features;

use features::{
    cards::*,
    deck::{deck_plugin::DeckPlugin, Deck},
};

fn main() {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugins(DeckPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    for (color, variant, transform) in [
        (
            CardColor::Blue,
            CardVariant::Number(9),
            Transform::from_xyz(-400., 0., 0.),
        ),
        (
            CardColor::Wild,
            CardVariant::Wild,
            Transform::from_xyz(0., 0., 0.),
        ),
        (
            CardColor::Yellow,
            CardVariant::Invert,
            Transform::from_xyz(400., 0., 0.),
        ),
    ] {
        let texture_path = CardBundle::texture_path(color, variant);

        let texture = asset_server.load(texture_path);
        commands.spawn((
            CardBundle { color, variant },
            SpriteBundle {
                texture,
                transform,
                ..default()
            },
        ));
    }
    commands.spawn(Deck::default());
}
