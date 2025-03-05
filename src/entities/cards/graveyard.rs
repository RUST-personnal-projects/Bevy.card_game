use bevy::prelude::*;

use crate::screens::{
    playing::game_loop::{PlayCardEvent, PlayCardOrderSet},
    Screen,
};

use super::Card;

#[derive(Component, Default)]
pub struct Graveyard {
    pub cards: Vec<Entity>,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (change_new_top_card)
            .in_set(PlayCardOrderSet::AddToGraveyard)
            .run_if(in_state(Screen::Playing)),
    );
}

fn change_new_top_card(
    mut ev_draw_card: EventReader<PlayCardEvent>,
    mut graveyard_query: Query<(&mut Graveyard, &Transform)>,
    mut cards_query: Query<&mut Transform, (With<Card>, Without<Graveyard>)>,
) {
    let (mut graveyard, graveyard_transform) = graveyard_query.single_mut();
    for &PlayCardEvent { card, .. } in ev_draw_card.read() {
        if let Some(&previous_top_card) = graveyard.cards.last() {
            let mut previous_top_card_transform = cards_query
                .get_mut(previous_top_card)
                .expect("Card in graveyard should exist.");
            previous_top_card_transform.translation.z = -1.;
        }
        graveyard.cards.push(card);
        if let Ok(mut card_transform) = cards_query.get_mut(card) {
            card_transform.translation.x = graveyard_transform.translation.x;
            card_transform.translation.y = graveyard_transform.translation.y;
        } else {
            panic!("Played card should exist");
        }
    }
}
