pub(crate) mod cards;
pub(crate) mod deck;
//pub mod screen;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        deck::plugin,
        //dev_tools::plugin,
    ));
}
