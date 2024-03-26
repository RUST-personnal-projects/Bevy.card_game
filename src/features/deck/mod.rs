pub mod deck_generator;
pub mod deck_plugin;

use bevy::prelude::*;

use crate::{CardColor, CardVariant};
pub use deck_generator::DeckGenerator;

pub type CardInfo = (CardColor, CardVariant);

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Deck(Vec<CardInfo>);

impl Default for Deck {
    fn default() -> Self {
        Self(DeckGenerator::default().generate_deck())
    }
}
