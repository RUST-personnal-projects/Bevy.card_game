use bevy::prelude::*;

use crate::{
    entities::cards::{deck::Deck, hand::Hand, InDeck},
    screens::Screen,
    utils::mouse::click::Clicked,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (draw_card, play_card)
            .run_if(in_state(Screen::Playing))
            .run_if(in_state(CurrentTurn::Player)),
    )
    .insert_state(CurrentTurn::Player);
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CurrentTurn {
    Player,
    _Adversary(u8),
}

fn draw_card(
    mut cards_in_deck_query: Query<&mut Visibility, With<InDeck>>,
    mut hand_query: Query<(Entity, &mut Hand), With<Hand>>,
    mut deck_query: Query<&mut Deck, With<Clicked>>,
    mut commands: Commands,
) {
    if let Ok(mut deck) = deck_query.get_single_mut() {
        if let Err(error) = deck.draw_card(&mut cards_in_deck_query, &mut hand_query, &mut commands)
        {
            warn!(error);
        }
    }
}

fn play_card() {}
