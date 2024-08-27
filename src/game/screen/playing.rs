//! The screen state for the main game loop.

use bevy::prelude::*;

use crate::{
    game::{
        card::CARD_BACK_PATH,
        deck::{DeckMarker, NodeDeckMarker, TextDeckMarker},
    },
    utils::UtilsBundle,
};

use bevy::color::palettes::css;

use super::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), spawn_deck);
}

fn spawn_deck(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a card back sprite representing the deck and an UI node containing text to show how many cards are left
    // UI node
    commands
        .spawn((
            NodeBundle {
                background_color: BackgroundColor(css::DARK_GRAY.into()),
                border_color: BorderColor(Color::BLACK),
                visibility: Visibility::Hidden,
                ..default()
            },
            NodeDeckMarker,
        ))
        .with_children(|builder| {
            builder.spawn((TextBundle::default(), TextDeckMarker));
        });

    let texture = asset_server.load(CARD_BACK_PATH);
    // Card back
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::new(0.5, 0.5, 1.)),
            texture,
            ..default()
        },
        UtilsBundle::default(),
        DeckMarker,
    ));
}
