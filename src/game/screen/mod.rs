pub(crate) mod playing;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        //     splash::plugin,
        //     loading::plugin,
        //     title::plugin,
        //     credits::plugin,
        playing::plugin,
        //     lose::plugin,
        //     win::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    _Splash,
    _Loading,
    _Title,
    _Credits,
    #[default]
    Playing,
    _Lose,
    _Win,
}
