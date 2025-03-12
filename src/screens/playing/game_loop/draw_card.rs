use bevy::prelude::*;

use super::turn::{CurrentPlayerState, CurrentTurnState};
use crate::{
    entities::cards::{deck::Deck, hand::Hand},
    screens::Screen,
    utils::mouse::{click::Clickable, hover::Hovered, on_clicked_event::OnEntityClickedEvent},
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
    mut draw_card_ev_writer: EventWriter<DrawCardEvent>,
    mut on_click_ev_reader: EventReader<OnEntityClickedEvent>,
    hand_query: Query<Entity, With<Hand>>,
    mut deck_query: Query<&mut Deck, (With<Clickable>, With<Hovered>)>,
    mut turn_next_state: ResMut<NextState<CurrentTurnState>>,
    mut commands: Commands,
) {
    for ev in on_click_ev_reader.read() {
        if let Ok(mut deck) = deck_query.get_mut(**ev) {
            if let Some(card) = deck.draw_card() {
                draw_card_ev_writer.send(DrawCardEvent {
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
}
