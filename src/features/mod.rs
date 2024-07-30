mod cards;
mod deck;
//pub mod screen;

use bevy::prelude::*;

pub(super) use cards::{CardBundle, CardColor, CardVariant};
pub(super) use deck::CARD_BACK_PATH;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        deck::plugin,
        //dev_tools::plugin,
    ));
}
