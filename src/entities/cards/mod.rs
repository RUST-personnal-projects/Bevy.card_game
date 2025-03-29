pub mod deck;
pub mod graveyard;
pub mod hand;

use bevy::prelude::*;

use crate::utils::assets::CardImageAssets;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((deck::plugin, hand::plugin, graveyard::plugin));
}

#[derive(Component)]
pub struct InDeck;

#[derive(Component)]
pub struct InGraveyard;

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
    pub fn map_to_asset_handle(&self, assets: &CardImageAssets) -> Handle<Image> {
        match self {
            Card::Wild(wild_variant) => match wild_variant {
                WildVariant::PlusFour => assets.plus_4.clone_weak(),
                WildVariant::ColorChange => assets.change_color.clone_weak(),
            },
            Card::Colored(colored_variant, card_color) => match (colored_variant, card_color) {
                (ColoredVariant::Number(0), CardColor::Yellow) => assets.yellow_0.clone_weak(),
                (ColoredVariant::Number(1), CardColor::Yellow) => assets.yellow_1.clone_weak(),
                (ColoredVariant::Number(2), CardColor::Yellow) => assets.yellow_2.clone_weak(),
                (ColoredVariant::Number(3), CardColor::Yellow) => assets.yellow_3.clone_weak(),
                (ColoredVariant::Number(4), CardColor::Yellow) => assets.yellow_4.clone_weak(),
                (ColoredVariant::Number(5), CardColor::Yellow) => assets.yellow_5.clone_weak(),
                (ColoredVariant::Number(6), CardColor::Yellow) => assets.yellow_6.clone_weak(),
                (ColoredVariant::Number(7), CardColor::Yellow) => assets.yellow_7.clone_weak(),
                (ColoredVariant::Number(8), CardColor::Yellow) => assets.yellow_8.clone_weak(),
                (ColoredVariant::Number(9), CardColor::Yellow) => assets.yellow_9.clone_weak(),
                (ColoredVariant::Number(0), CardColor::Red) => assets.red_0.clone_weak(),
                (ColoredVariant::Number(1), CardColor::Red) => assets.red_1.clone_weak(),
                (ColoredVariant::Number(2), CardColor::Red) => assets.red_2.clone_weak(),
                (ColoredVariant::Number(3), CardColor::Red) => assets.red_3.clone_weak(),
                (ColoredVariant::Number(4), CardColor::Red) => assets.red_4.clone_weak(),
                (ColoredVariant::Number(5), CardColor::Red) => assets.red_5.clone_weak(),
                (ColoredVariant::Number(6), CardColor::Red) => assets.red_6.clone_weak(),
                (ColoredVariant::Number(7), CardColor::Red) => assets.red_7.clone_weak(),
                (ColoredVariant::Number(8), CardColor::Red) => assets.red_8.clone_weak(),
                (ColoredVariant::Number(9), CardColor::Red) => assets.red_9.clone_weak(),
                (ColoredVariant::Number(0), CardColor::Blue) => assets.blue_0.clone_weak(),
                (ColoredVariant::Number(1), CardColor::Blue) => assets.blue_1.clone_weak(),
                (ColoredVariant::Number(2), CardColor::Blue) => assets.blue_2.clone_weak(),
                (ColoredVariant::Number(3), CardColor::Blue) => assets.blue_3.clone_weak(),
                (ColoredVariant::Number(4), CardColor::Blue) => assets.blue_4.clone_weak(),
                (ColoredVariant::Number(5), CardColor::Blue) => assets.blue_5.clone_weak(),
                (ColoredVariant::Number(6), CardColor::Blue) => assets.blue_6.clone_weak(),
                (ColoredVariant::Number(7), CardColor::Blue) => assets.blue_7.clone_weak(),
                (ColoredVariant::Number(8), CardColor::Blue) => assets.blue_8.clone_weak(),
                (ColoredVariant::Number(9), CardColor::Blue) => assets.blue_9.clone_weak(),
                (ColoredVariant::Number(0), CardColor::Green) => assets.green_0.clone_weak(),
                (ColoredVariant::Number(1), CardColor::Green) => assets.green_1.clone_weak(),
                (ColoredVariant::Number(2), CardColor::Green) => assets.green_2.clone_weak(),
                (ColoredVariant::Number(3), CardColor::Green) => assets.green_3.clone_weak(),
                (ColoredVariant::Number(4), CardColor::Green) => assets.green_4.clone_weak(),
                (ColoredVariant::Number(5), CardColor::Green) => assets.green_5.clone_weak(),
                (ColoredVariant::Number(6), CardColor::Green) => assets.green_6.clone_weak(),
                (ColoredVariant::Number(7), CardColor::Green) => assets.green_7.clone_weak(),
                (ColoredVariant::Number(8), CardColor::Green) => assets.green_8.clone_weak(),
                (ColoredVariant::Number(9), CardColor::Green) => assets.green_9.clone_weak(),
                (ColoredVariant::Invert, CardColor::Yellow) => assets.green_7.clone_weak(),
                (ColoredVariant::Invert, CardColor::Red) => assets.red_inverse.clone_weak(),
                (ColoredVariant::Invert, CardColor::Blue) => assets.blue_inverse.clone_weak(),
                (ColoredVariant::Invert, CardColor::Green) => assets.green_inverse.clone_weak(),
                (ColoredVariant::Block, CardColor::Yellow) => assets.yellow_inverse.clone_weak(),
                (ColoredVariant::Block, CardColor::Red) => assets.red_block.clone_weak(),
                (ColoredVariant::Block, CardColor::Blue) => assets.red_block.clone_weak(),
                (ColoredVariant::Block, CardColor::Green) => assets.red_block.clone_weak(),
                (ColoredVariant::PlusTwo, CardColor::Yellow) => assets.yellow_plus_two.clone_weak(),
                (ColoredVariant::PlusTwo, CardColor::Red) => assets.red_plus_two.clone_weak(),
                (ColoredVariant::PlusTwo, CardColor::Blue) => assets.blue_plus_two.clone_weak(),
                (ColoredVariant::PlusTwo, CardColor::Green) => assets.green_plus_two.clone_weak(),
                _ => panic!("This card variand doesn't exist"),
            },
        }
    }

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
