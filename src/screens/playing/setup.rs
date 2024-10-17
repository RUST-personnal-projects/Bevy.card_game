use bevy::prelude::*;

use crate::{
    entities::{
        cards::deck::{Deck, DeckMarker, NodeDeckMarker, TextDeckMarker},
        fixed_entities::{FixedPosition, FixedScale},
    },
    screens::Screen,
    utils::{
        assets::{images::ImageKey, HandleMap},
        UtilsBundle,
    },
};

use bevy::color::palettes::css;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Playing),
        (spawn_deck, spawn_graveyard, spawn_hand),
    );
}

fn spawn_deck(mut commands: Commands, image_handles: Res<HandleMap<ImageKey>>) {
    // Spawn an UI node containing text to show how many cards are left
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

    let texture_handle = image_handles
        .get(&ImageKey::CardBack)
        .expect("Cardback should be set");

    // Spawn the game deck including it's sprite
    commands.spawn((
        SpriteBundle {
            texture: texture_handle.clone(),
            ..default()
        },
        UtilsBundle::default(),
        Deck::default(),
        FixedPosition::DECK,
        FixedScale::CARD,
        DeckMarker,
    ));
}

fn spawn_graveyard(mut commands: Commands, image_handles: Res<HandleMap<ImageKey>>) {
    let texture_handle = image_handles
        .get(&ImageKey::CardBack)
        .expect("Cardback should be set");

    // Spawn the game deck including it's sprite
    commands.spawn((
        SpriteBundle {
            texture: texture_handle.clone(),
            ..default()
        },
        UtilsBundle::default(),
        FixedPosition::GRAVEYARD,
        FixedScale::CARD,
    ));
}

fn spawn_hand(mut commands: Commands, image_handles: Res<HandleMap<ImageKey>>) {
    let texture_handle = image_handles
        .get(&ImageKey::CardBack)
        .expect("Cardback should be always set statically");

    // Spawn the game deck including it's sprite
    commands.spawn((
        SpriteBundle {
            texture: texture_handle.clone(),
            ..default()
        },
        UtilsBundle::default(),
        FixedPosition::HAND,
        FixedScale::CARD,
    ));
}
