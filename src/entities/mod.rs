pub(crate) mod cards;
pub(crate) mod window_resize;

use bevy::prelude::*;
use window_resize::{Scalable, Translatable};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((cards::plugin, window_resize::plugin));
}

#[derive(Component, Debug)]
pub(crate) enum GameEntity {
    Deck,
    Cemetery,
    Hand,
}

impl Translatable for GameEntity {
    fn get_translation(&self) -> Vec2 {
        match self {
            GameEntity::Deck => Vec2::new(0., 0.),
            GameEntity::Cemetery => Vec2::new(-0.25, 0.),
            GameEntity::Hand => Vec2::new(0., -0.5),
        }
    }
}

impl Scalable for GameEntity {
    fn scale(&self) -> Vec2 {
        match self {
            GameEntity::Deck => Vec2::new(0.1, 0.2),
            GameEntity::Cemetery => Vec2::new(0.1, 0.2),
            GameEntity::Hand => Vec2::new(0.8, 0.2),
        }
    }
}
