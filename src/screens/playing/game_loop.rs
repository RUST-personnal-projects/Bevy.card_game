use bevy::prelude::*;

use crate::{
    entities::cards::{deck::Deck, hand::Hand, Card, InHand},
    screens::Screen,
    utils::mouse::click::Clicked,
};

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CurrentTurn {
    Player,
    _Adversary(u8),
}

#[derive(Event)]
pub struct DrawCardEvent {
    pub card: Entity,
    pub hand: Entity,
}

#[derive(Event)]
pub struct PlayCardEvent {
    pub card: Entity,
    pub hand: Entity,
}

// This system set serves as a way to choose in which order dev info are printed
// When adding a new member to the set, don't forget to also add it to the configure_sets down bellow
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayCardOrderSet {
    RemoveFromHand,
    AddToGraveyard,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (send_draw_card_event, send_play_card_event)
            .run_if(in_state(Screen::Playing))
            .run_if(in_state(CurrentTurn::Player)),
    )
    .configure_sets(
        Update,
        (
            PlayCardOrderSet::RemoveFromHand,
            PlayCardOrderSet::AddToGraveyard,
        )
            .chain(),
    )
    .add_event::<DrawCardEvent>()
    .add_event::<PlayCardEvent>()
    .insert_state(CurrentTurn::Player);
}

fn send_draw_card_event(
    mut ev_draw_card: EventWriter<DrawCardEvent>,
    hand_query: Query<Entity, With<Hand>>,
    mut deck_query: Query<&mut Deck, Added<Clicked>>,
) {
    if let Ok(mut deck) = deck_query.get_single_mut() {
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

fn send_play_card_event(
    mut ev_draw_card: EventWriter<PlayCardEvent>,
    mut hand_query: Query<(Entity, &mut Hand)>,
    card_query: Query<Entity, (Added<Clicked>, With<InHand>, With<Card>)>,
) {
    if let Ok((hand, mut hand_component)) = hand_query.get_single_mut() {
        if let Some(card) = hand_component.play_card(card_query.get_single().ok()) {
            ev_draw_card.send(PlayCardEvent { card, hand });
        }
    } else {
        warn!("Tried to play card from non-existing hand");
    }
}
