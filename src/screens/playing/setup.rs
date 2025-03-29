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
        assets::CardImageAssets,
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

fn spawn_deck(mut commands: Commands, card_assets: Res<CardImageAssets>) {
    // Spawn an UI node containing text to show how many cards are left
    commands
        .spawn((
            Node::default(),
            BackgroundColor(css::DARK_GRAY.into()),
            BorderColor(Color::BLACK),
            Visibility::Hidden,
            NodeDeckMarker,
        ))
        .with_children(|builder| {
            builder.spawn((Text::default(), TextDeckMarker));
        });

    let mut deck = Deck::default();

    // Spawn the game deck including it's sprite
    commands
        .spawn((
            Sprite::from_image(card_assets.card_back.clone_weak()),
            UtilsBundle::default(),
            FixedPosition::DECK,
            FixedScale::CARD,
            DeckMarker,
        ))
        .with_children(|parent| {
            let cards = DeckGenerator::default().generate_deck();
            cards.iter().for_each(|&card| {
                let card_entity = parent
                    .spawn((
                        card,
                        InDeck,
                        Sprite::from_image(card.map_to_asset_handle(&card_assets)),
                        Visibility::Hidden,
                        FixedScale::CARD,
                        UtilsBundle::default(),
                    ))
                    .id();
                deck.0.push_front(card_entity);
            });
        })
        .insert(deck);
}

fn spawn_graveyard(mut commands: Commands, card_assets: Res<CardImageAssets>) {
    // Spawn the game graveyard including it's sprite
    commands.spawn((
        Sprite::from_image(card_assets.card_back.clone_weak()),
        Transform::from_translation(Vec3::new(0., 0., -1.)),
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
        Transform::default(),
        Visibility::default(),
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
