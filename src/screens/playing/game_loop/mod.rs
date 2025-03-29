pub mod draw_card;
pub mod play_card;
pub mod turn;

use bevy::prelude::*;

use crate::screens::Screen;
use turn::{CurrentPlayerState, CurrentTurnState};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((play_card::plugin, draw_card::plugin, turn::plugin))
        .add_systems(
            Update,
            (end_turn)
                .run_if(in_state(Screen::Playing))
                .run_if(in_state(CurrentPlayerState::LocalPlayer))
                .run_if(in_state(CurrentTurnState::End)),
        );
}

fn end_turn(mut turn_next_state: ResMut<NextState<CurrentTurnState>>) {
    turn_next_state.set(CurrentTurnState::Start);
}

// fn pass_turn() {}
