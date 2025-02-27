use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

use crate::{
    screens::Screen,
    utils::image_scaling::{ScaleOrderSet, ScaledSize},
};

use super::Card;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (reposition_hand_on_change, reposition_hand_on_window_resize)
            .in_set(ScaleOrderSet::UseScaledSize)
            .run_if(in_state(Screen::Playing)),
    );
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct Hand {
    pub cards: Vec<Entity>,
}

#[derive(Component)]
pub struct PlayerHand;

fn reposition_hand_on_change(
    hand_query: Query<&Hand, (Changed<Hand>, With<PlayerHand>)>,
    mut cards_query: Query<(Entity, &mut Transform, &ScaledSize), With<Card>>,
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
    mut cards_query: Query<(Entity, &mut Transform, &ScaledSize), With<Card>>,
    mut events: EventReader<WindowResized>,
) {
    for WindowResized { width, .. } in events.read() {
        if let Ok(hand) = hand_query.get_single() {
            reposition_hand(hand, &mut cards_query, *width);
        }
    }
}

fn reposition_hand(
    hand: &Hand,
    cards_query: &mut Query<(Entity, &mut Transform, &ScaledSize), With<Card>>,
    window_width: f32,
) {
    let card_count = hand.cards.len();
    for (i, &card_entity) in hand.cards.iter().enumerate() {
        if let Ok((_, mut transform, scaled_size)) = cards_query.get_mut(card_entity) {
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
