use bevy::prelude::*;

use crate::dev_tools::{DebugViewOrderSet, DevState};

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CurrentPlayerState {
    LocalPlayer,
    _Adversary(u8),
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CurrentTurnState {
    Start,
    Drawn,
    End,
}

pub(super) fn plugin(app: &mut App) {
    #[cfg(feature = "dev")]
    app.add_systems(
        Startup,
        debug::setup_turn_debug_view.in_set(DebugViewOrderSet::Turn),
    )
    .add_systems(
        Update,
        debug::update_current_turn.run_if(in_state(DevState::On)),
    );
}

#[cfg(feature = "dev")]
mod debug {
    use super::*;

    use crate::dev_tools::DebugNodeMarker;

    #[derive(Component)]
    pub(super) struct TurnMarker;

    pub(super) fn setup_turn_debug_view(
        mut commands: Commands,
        debug_node_query: Query<Entity, With<DebugNodeMarker>>,
    ) {
        let node = debug_node_query.single();

        let turn = commands.spawn((Text::default(), TurnMarker)).id();

        commands.entity(node).add_children(&[turn]);
    }

    pub(super) fn update_current_turn(
        turn: Res<State<CurrentTurnState>>,
        mut text_query: Query<&mut Text, With<TurnMarker>>,
    ) {
        let mut turn_text = text_query.single_mut();
        *turn_text = Text::new(format!("Turn: {:?}", turn.get()));
    }
}
