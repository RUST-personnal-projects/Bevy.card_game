use bevy::prelude::*;

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
