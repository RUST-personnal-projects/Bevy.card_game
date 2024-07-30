mod click;
mod coordinates;
mod hover;

use bevy::prelude::*;

pub use self::{
    click::{Clickable, Clicked},
    coordinates::{MouseCoordinates, UIMouseCoordinates},
    hover::{Hoverable, Hovered},
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((click::plugin, hover::plugin, coordinates::plugin));
}

#[derive(Bundle, Debug, Default)]
pub struct MouseInteractionBundle {
    pub clickable: Clickable,
    pub hoverable: Hoverable,
}
