use bevy::prelude::*;

use super::turn::{CurrentPlayerState, CurrentTurnState};
use crate::{
    entities::cards::{deck::Deck, hand::Hand},
    screens::Screen,
    utils::mouse::click::Clicked,
};

#[derive(Event)]
pub struct DrawCardEvent {
    pub card: Entity,
    pub hand: Entity,
}

#[derive(Component)]
pub struct DrawnCardMarker;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (send_draw_card_event)
            .run_if(in_state(Screen::Playing))
            .run_if(in_state(CurrentPlayerState::LocalPlayer))
            .run_if(in_state(CurrentTurnState::Start)),
    )
    .add_event::<DrawCardEvent>();
}

fn send_draw_card_event(
    mut ev_draw_card: EventWriter<DrawCardEvent>,
    hand_query: Query<Entity, With<Hand>>,
    mut deck_query: Query<&mut Deck, Added<Clicked>>,
    mut turn_next_state: ResMut<NextState<CurrentTurnState>>,
    mut commands: Commands,
) {
    if let Ok(mut deck) = deck_query.get_single_mut() {
        if let Some(card) = deck.draw_card() {
            ev_draw_card.send(DrawCardEvent {
                card,
                hand: hand_query.single(),
            });
            commands.entity(card).insert(DrawnCardMarker);
            turn_next_state.set(CurrentTurnState::Drawn);
        } else {
            warn!("Tried to remove card from empty deck");
        }
    }
}
