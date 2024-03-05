use bevy::prelude::*;

mod features;

use features::cards::*;

fn main() {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugins(CardsPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    for (color, variant, transform) in [
        (
            CardColor::Blue,
            CardVariant::Number(CardValue::Nine),
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
        let texture_path = Card::texture_path(color, variant);

        let texture = asset_server.load(texture_path);
        commands.spawn((
            Card::default(),
            SpriteBundle {
                texture,
                transform,
                ..default()
            },
        ));
    }
}
