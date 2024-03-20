pub mod click;
pub mod coordinates;
pub mod hover;

use bevy::prelude::*;

pub use self::{
    click::{Clickable, Clicked},
    coordinates::MouseCoordinates,
    hover::{Hoverable, Hovered},
};

pub struct MousePlugins;

impl PluginGroup for MousePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add(click::ClickPlugin)
            .add(hover::HoverPlugin)
            .add(coordinates::CoordinatesPlugin)
    }
}

#[derive(Bundle, Debug, Default)]
pub struct MouseInteractionBundle {
    pub clickable: Clickable,
    pub hoverable: Hoverable,
}
