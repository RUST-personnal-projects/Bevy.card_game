use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    entities::{
        cards::deck::{Deck, DeckMarker, NodeDeckMarker, TextDeckMarker},
        window_resize::{Scalable, Translatable},
        GameEntity,
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
        (spawn_deck, spawn_cemetery, spawn_hand),
    );
}

fn spawn_deck(
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    images: Res<Assets<Image>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
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

    let window = window_query.single();
    let deck = GameEntity::Deck;
    let (texture_handle, texture) = image_handles
        .get(&ImageKey::CardBack)
        .map(|handle| {
            (
                handle,
                images.get(handle).expect("Cardback should be loaded"),
            )
        })
        .expect("Cardback should be set");
    let translation = deck
        .update_translation(window.width(), window.height())
        .extend(1.);
    let scale = deck
        .update_scale(window.width(), window.height(), texture.size_f32())
        .extend(1.);

    // Spawn the game deck including it's sprite
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(translation).with_scale(scale),
            texture: texture_handle.clone(),
            ..default()
        },
        UtilsBundle::default(),
        Deck::default(),
        DeckMarker,
        deck,
    ));
}

fn spawn_cemetery(
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    images: Res<Assets<Image>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let cemetery = GameEntity::Cemetery;
    let (texture_handle, texture) = image_handles
        .get(&ImageKey::CardBack)
        .map(|handle| {
            (
                handle,
                images.get(handle).expect("Cardback should be loaded"),
            )
        })
        .expect("Cardback should be set");
    let translation = cemetery
        .update_translation(window.width(), window.height())
        .extend(1.);
    let scale = cemetery
        .update_scale(window.width(), window.height(), texture.size_f32())
        .extend(1.);

    // Spawn the game deck including it's sprite
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(translation).with_scale(scale),
            texture: texture_handle.clone(),
            ..default()
        },
        UtilsBundle::default(),
        Deck::default(),
        cemetery,
    ));
}

fn spawn_hand(
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    images: Res<Assets<Image>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let hand = GameEntity::Hand;
    let (texture_handle, texture) = image_handles
        .get(&ImageKey::CardBack)
        .map(|handle| {
            (
                handle,
                images.get(handle).expect("Cardback should be loaded"),
            )
        })
        .expect("Cardback should be set");
    let translation = hand
        .update_translation(window.width(), window.height())
        .extend(1.);
    let scale = hand
        .update_scale(window.width(), window.height(), texture.size_f32())
        .extend(1.);

    // Spawn the game deck including it's sprite
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(translation).with_scale(scale),
            texture: texture_handle.clone_weak(),
            ..default()
        },
        UtilsBundle::default(),
        hand,
    ));
}
