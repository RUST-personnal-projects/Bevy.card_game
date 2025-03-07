use bevy::prelude::*;

use super::{
    draw_card::DrawnCardMarker,
    turn::{CurrentPlayerState, CurrentTurnState},
};
use crate::{
    entities::cards::{hand::Hand, Card, InHand},
    screens::Screen,
    utils::mouse::click::Clicked,
};

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
    ChangeState,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (send_play_card_event)
            .run_if(in_state(Screen::Playing))
            .run_if(in_state(CurrentPlayerState::LocalPlayer))
            .run_if(in_state(CurrentTurnState::Start)),
    )
    .add_systems(
        Update,
        (send_play_drawn_card_event)
            .run_if(in_state(Screen::Playing))
            .run_if(in_state(CurrentPlayerState::LocalPlayer))
            .run_if(in_state(CurrentTurnState::Drawn)),
    )
    .configure_sets(
        Update,
        (
            PlayCardOrderSet::RemoveFromHand,
            PlayCardOrderSet::AddToGraveyard,
            PlayCardOrderSet::ChangeState,
        )
            .chain(),
    )
    .add_event::<PlayCardEvent>()
    .insert_state(CurrentPlayerState::LocalPlayer);
}

fn send_play_card_event(
    mut ev_draw_card: EventWriter<PlayCardEvent>,
    mut hand_query: Query<(Entity, &mut Hand)>,
    card_query: Query<Entity, (Added<Clicked>, With<InHand>, With<Card>)>,
    mut turn_next_state: ResMut<NextState<CurrentTurnState>>,
) {
    if let Ok((hand, mut hand_component)) = hand_query.get_single_mut() {
        if let Some(card) = hand_component.play_card(card_query.get_single().ok()) {
            ev_draw_card.send(PlayCardEvent { card, hand });
        }
        turn_next_state.set(CurrentTurnState::End);
    } else {
        warn!("Tried to play card from non-existing hand");
    }
}

fn send_play_drawn_card_event(
    mut ev_draw_card: EventWriter<PlayCardEvent>,
    mut hand_query: Query<(Entity, &mut Hand)>,
    card_query: Query<
        Entity,
        (
            Added<Clicked>,
            With<InHand>,
            With<Card>,
            With<DrawnCardMarker>,
        ),
    >,
    mut turn_next_state: ResMut<NextState<CurrentTurnState>>,
    mut commands: Commands,
) {
    if let Ok((hand, mut hand_component)) = hand_query.get_single_mut() {
        if let Some(card) = hand_component.play_card(card_query.get_single().ok()) {
            ev_draw_card.send(PlayCardEvent { card, hand });
            commands.entity(card).remove::<DrawnCardMarker>();
            turn_next_state.set(CurrentTurnState::End);
        }
    } else {
        warn!("Tried to play card from non-existing hand");
    }
}
