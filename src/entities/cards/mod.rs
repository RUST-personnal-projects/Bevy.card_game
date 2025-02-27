pub mod deck;
pub mod graveyard;
pub mod hand;

use bevy::prelude::*;

pub const CARD_BACK_PATH: &str = "images/cards/card_back.png";

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((deck::plugin, hand::plugin));
}

#[derive(Component)]
pub struct InDeck;

#[derive(Component)]
pub struct _InGraveyard;

#[derive(Component)]
pub struct InHand;

#[derive(Debug, Clone, Copy, PartialEq, Component, Hash, Eq, Reflect)]
pub enum Card {
    Colored(ColoredVariant, CardColor),
    Wild(WildVariant),
}

impl From<Card> for String {
    fn from(card: Card) -> String {
        match card {
            Card::Colored(variant, color) => {
                format!("{}_{}", String::from(variant), String::from(color))
            }
            Card::Wild(variant) => String::from(variant),
        }
    }
}

impl Card {
    /// Automatically retrieve a list of all possible variations of cards
    ///
    /// Possible usage: get a list of all different card assets to load
    pub fn all_variations() -> Vec<Self> {
        let colors = CardColor::all_variations();
        let colored_variants = ColoredVariant::all_variations();
        let wild_variants = WildVariant::all_variations();
        let colored: Vec<Card> = colored_variants
            .into_iter()
            .flat_map(|variant| {
                colors
                    .clone()
                    .into_iter()
                    .map(move |color| Self::Colored(variant, color))
            })
            .collect();
        let wild: Vec<Card> = wild_variants.into_iter().map(Self::Wild).collect();
        [colored, wild].concat()
    }

    pub fn texture_path(self) -> String {
        match self {
            Self::Colored(_, color) => {
                format!(
                    "images/cards/{}/{}.png",
                    String::from(color),
                    String::from(self)
                )
            }
            Self::Wild(_) => format!("images/cards/{}.png", String::from(self)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Reflect)]
pub enum CardColor {
    Yellow,
    Red,
    Blue,
    Green,
}

impl From<CardColor> for String {
    fn from(card_color: CardColor) -> String {
        match card_color {
            CardColor::Yellow => "yellow".to_string(),
            CardColor::Red => "red".to_string(),
            CardColor::Blue => "blue".to_string(),
            CardColor::Green => "green".to_string(),
        }
    }
}

impl CardColor {
    pub fn all_variations() -> Vec<Self> {
        vec![Self::Yellow, Self::Red, Self::Blue, Self::Green]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Reflect)]
pub enum ColoredVariant {
    Number(u8),
    Invert,
    Block,
    PlusTwo,
}

impl From<ColoredVariant> for String {
    fn from(card_variant: ColoredVariant) -> String {
        match card_variant {
            ColoredVariant::Number(number) => number.to_string(),
            ColoredVariant::Invert => "inverse".to_string(),
            ColoredVariant::Block => "block".to_string(),
            ColoredVariant::PlusTwo => "2plus".to_string(),
        }
    }
}

impl ColoredVariant {
    pub fn all_variations() -> Vec<Self> {
        let numbers: Vec<Self> = (0..10).map(Self::Number).collect();
        [vec![Self::Invert, Self::Block, Self::PlusTwo], numbers].concat()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Reflect)]
pub enum WildVariant {
    PlusFour,
    ColorChange,
}

impl From<WildVariant> for String {
    fn from(card_variant: WildVariant) -> String {
        match card_variant {
            WildVariant::PlusFour => "4_plus".to_string(),
            WildVariant::ColorChange => "color_change".to_string(),
        }
    }
}

impl WildVariant {
    pub fn all_variations() -> Vec<Self> {
        vec![Self::PlusFour, Self::ColorChange]
    }
}
