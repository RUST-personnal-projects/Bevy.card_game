pub mod click;
pub mod coordinates;
pub mod hover;
pub mod on_clicked_event;

use bevy::prelude::*;

use click::Clickable;
use hover::Hoverable;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        click::plugin,
        hover::plugin,
        coordinates::plugin,
        on_clicked_event::plugin,
    ));
}

#[derive(Bundle, Debug, Default)]
pub struct MouseInteractionBundle {
    pub clickable: Clickable,
    pub hoverable: Hoverable,
}
