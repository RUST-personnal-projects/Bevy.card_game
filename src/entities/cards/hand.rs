use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

use crate::{
    screens::{
        playing::game_loop::{
            draw_card::DrawCardEvent,
            play_card::{PlayCardEvent, PlayCardOrderSet},
        },
        Screen,
    },
    utils::image_scaling::{ScaleOrderSet, ScaledSize},
};

use super::{Card, InDeck, InGraveyard, InHand};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (reposition_hand_on_change, reposition_hand_on_window_resize)
            .in_set(ScaleOrderSet::UseScaledSize)
            .run_if(in_state(Screen::Playing)),
    )
    .add_systems(
        Update,
        (read_play_card_event)
            .in_set(PlayCardOrderSet::RemoveFromHand)
            .run_if(in_state(Screen::Playing)),
    )
    .add_systems(
        Update,
        (read_draw_card_event).run_if(in_state(Screen::Playing)),
    );
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct Hand {
    pub cards: Vec<Entity>,
}

impl Hand {
    pub fn play_card(&mut self, played_card: Option<Entity>) -> Option<Entity> {
        let played_card = played_card?;
        Some(
            self.cards
                .remove(self.cards.iter().position(|&card| card == played_card)?),
        )
    }
}

#[derive(Component)]
pub struct PlayerHand;

fn reposition_hand_on_change(
    hand_query: Query<&Hand, (Changed<Hand>, With<PlayerHand>)>,
    mut cards_query: Query<(&mut Transform, &ScaledSize), With<Card>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let window_width = window.width();
    if let Ok(hand) = hand_query.get_single() {
        reposition_hand(hand, &mut cards_query, window_width);
    }
}

fn reposition_hand_on_window_resize(
    hand_query: Query<&Hand, With<PlayerHand>>,
    mut cards_query: Query<(&mut Transform, &ScaledSize), With<Card>>,
    mut events: EventReader<WindowResized>,
) {
    for &WindowResized {
        width: window_width,
        ..
    } in events.read()
    {
        if let Ok(hand) = hand_query.get_single() {
            reposition_hand(hand, &mut cards_query, window_width);
        }
    }
}

fn reposition_hand(
    hand: &Hand,
    cards_query: &mut Query<(&mut Transform, &ScaledSize), With<Card>>,
    window_width: f32,
) {
    let card_count = hand.cards.len();
    for (i, &card_entity) in hand.cards.iter().enumerate() {
        if let Ok((mut transform, scaled_size)) = cards_query.get_mut(card_entity) {
            let spacing = scaled_size.x + (window_width * 0.01); // ratio of window width

            // Calculate the starting position to center the cards
            let start_x = -((card_count as f32 - 1.0) * spacing) / 2.0; // Centering calculation

            // Set the card's position relative to the Hand entity's position
            let x_position = start_x + (i as f32 * (spacing));

            // Update the transform position for X only
            transform.translation.x = x_position;
            // Note: Y is not set, as it will be inherited from the Hand
        }
    }
}

fn read_draw_card_event(
    mut ev_draw_card: EventReader<DrawCardEvent>,
    mut hand_query: Query<&mut Hand>,
    mut cards_in_deck_query: Query<&mut Visibility, With<InDeck>>,
    mut commands: Commands,
) {
    for &DrawCardEvent { card, hand } in ev_draw_card.read() {
        let mut card_visibility = cards_in_deck_query
            .get_mut(card)
            .unwrap_or_else(|err| panic!("Error retrieving card: {err}"));

        if let Ok(mut hand_vec) = hand_query.get_mut(hand) {
            commands.entity(card).remove::<InDeck>();
            *card_visibility = Visibility::Visible;
            commands.entity(card).set_parent(hand);
            hand_vec.cards.push(card);
            commands.entity(card).insert(InHand);
        } else {
            panic!("Hand doesn't exist");
        }
    }
}

fn read_play_card_event(mut ev_draw_card: EventReader<PlayCardEvent>, mut commands: Commands) {
    for &PlayCardEvent { card, hand: _hand } in ev_draw_card.read() {
        commands.entity(card).remove::<InHand>();
        commands.entity(card).remove_parent();
        commands.entity(card).insert(InGraveyard);
    }
}
