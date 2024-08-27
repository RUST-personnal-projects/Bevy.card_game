//! The screen state for the main game loop.

use bevy::prelude::*;

use crate::{
    game::{
        card::{Card, CardColor, ColoredVariant, WildVariant, CARD_BACK_PATH},
        deck::{DeckMarker, NodeDeckMarker, TextDeckMarker},
    },
    utils::mouse::MouseInteractionBundle,
};

use bevy::color::palettes::css;

use crate::utils::mouse::{click::Clickable, hover::Hoverable};

use super::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), enter_playing);
}

fn enter_playing(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            transform: Transform::from_xyz(0., 300., 0.),
            texture,
            ..default()
        },
        DeckMarker,
        Hoverable,
        Clickable,
    ));
}
