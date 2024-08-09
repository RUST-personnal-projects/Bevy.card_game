use bevy::prelude::*;

pub(crate) const CARD_BACK_PATH: &str = "cards/card_back.png";

#[derive(Debug, Clone, Copy, PartialEq, Component, Hash, Eq, Reflect)]
pub(crate) enum CardColor {
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
    pub(crate) fn all_variations() -> Vec<Self> {
        vec![Self::Yellow, Self::Red, Self::Blue, Self::Green]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Component, Hash, Eq, Reflect)]
pub(crate) enum ColoredVariant {
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
    pub(crate) fn all_variations() -> Vec<Self> {
        let numbers: Vec<Self> = (0..10).map(Self::Number).collect();
        [vec![Self::Invert, Self::Block, Self::PlusTwo], numbers].concat()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Component, Hash, Eq, Reflect)]
pub(crate) enum WildVariant {
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
    pub(crate) fn all_variations() -> Vec<Self> {
        vec![Self::PlusFour, Self::ColorChange]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Component, Hash, Eq, Reflect)]
pub(crate) enum Card {
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
    pub(crate) fn all_variations() -> Vec<Self> {
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

    pub(crate) fn texture_path(self) -> String {
        match self {
            Self::Colored(_, color) => {
                format!("cards/{}/{}.png", String::from(color), String::from(self))
            }
            Self::Wild(_) => format!("cards/{}.png", String::from(self)),
        }
    }
}

// #[derive(Bundle, Debug, Clone)]
// pub struct CardBundle {
//     pub color: CardColor,
//     pub variant: CardVariant,
// }

// impl CardBundle {
//     pub fn texture_path(color: CardColor, variant: CardVariant) -> String {
//         let color = String::from(color);
//         format!("cards/{}/{}_{}.png", color, String::from(variant), color)
//     }
// }
