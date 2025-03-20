use bevy::prelude::*;

use crate::{
    entities::{
        cards::{
            deck::{generator::DeckGenerator, Deck, DeckMarker, NodeDeckMarker, TextDeckMarker},
            graveyard::Graveyard,
            hand::{Hand, PlayerHand},
            InDeck,
        },
        fixed_entities::{FixedPosition, FixedScale},
    },
    screens::Screen,
    utils::{
        assets::{images::ImageKey, HandleMap},
        // ui::prelude::*,
        UtilsBundle,
    },
};

use bevy::color::palettes::css;

use super::game_loop::{
    draw_card::DrawCardEvent,
    turn::{CurrentPlayerState, CurrentTurnState},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Playing),
        (spawn_deck, spawn_graveyard, spawn_hand, draw_hand, spawn_ui).chain(),
    )
    .insert_state(CurrentTurnState::Start)
    .insert_state(CurrentPlayerState::LocalPlayer);
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

    let mut deck = Deck::default();

    // Spawn the game deck including it's sprite
    commands
        .spawn((
            SpriteBundle {
                texture: texture_handle.clone_weak(),
                ..default()
            },
            UtilsBundle::default(),
            FixedPosition::DECK,
            FixedScale::CARD,
            DeckMarker,
        ))
        .with_children(|parent| {
            let cards = DeckGenerator::default().generate_deck();
            cards.iter().for_each(|&card| {
                let texture_handle = image_handles
                    .get(&ImageKey::Card(card))
                    .unwrap_or_else(|| panic!("{:?} should be set", card));
                let card_entity = parent
                    .spawn((
                        card,
                        InDeck,
                        SpriteBundle {
                            visibility: Visibility::Hidden,
                            texture: texture_handle.clone_weak(),
                            ..default()
                        },
                        FixedScale::CARD,
                        UtilsBundle::default(),
                    ))
                    .id();
                deck.0.push_front(card_entity);
            });
        })
        .insert(deck);
}

fn spawn_graveyard(mut commands: Commands, image_handles: Res<HandleMap<ImageKey>>) {
    let texture_handle = image_handles
        .get(&ImageKey::CardBack)
        .expect("Cardback should be set");

    // Spawn the game graveyard including it's sprite
    commands.spawn((
        SpriteBundle {
            texture: texture_handle.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., -1.),
                ..default()
            },
            ..default()
        },
        UtilsBundle::default(),
        FixedPosition::GRAVEYARD,
        FixedScale::CARD,
        Graveyard::default(),
    ));
}

fn spawn_hand(mut commands: Commands) {
    commands.spawn((
        FixedPosition::HAND,
        Hand::default(),
        PlayerHand,
        SpatialBundle::default(),
    ));
}

fn spawn_ui(mut _commands: Commands) {
    // commands.ui_root().with_children(|children| {
    //     children.button("button 1", None, None);
    //     children.button("button 2", None, None);
    // });
}

fn draw_hand(
    mut ev_draw_card: EventWriter<DrawCardEvent>,
    hand_query: Query<Entity, With<Hand>>,
    mut deck_query: Query<&mut Deck>,
) {
    let mut deck = deck_query.single_mut();

    for _ in 0..7 {
        if let Some(card) = deck.draw_card() {
            ev_draw_card.send(DrawCardEvent {
                card,
                hand: hand_query.single(),
            });
        } else {
            warn!("Tried to remove card from empty deck");
        }
    }
}
