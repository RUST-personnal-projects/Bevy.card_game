use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub enum CardColor {
    Yellow,
    Red,
    Blue,
    Green,
    Wild,
}

impl From<CardColor> for &'static str {
    fn from(card_color: CardColor) -> &'static str {
        match card_color {
            CardColor::Yellow => "yellow",
            CardColor::Red => "red",
            CardColor::Blue => "blue",
            CardColor::Green => "green",
            CardColor::Wild => "wild",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CardValue {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl From<CardValue> for &'static str {
    fn from(card_value: CardValue) -> &'static str {
        match card_value {
            CardValue::One => "1",
            CardValue::Two => "2",
            CardValue::Three => "3",
            CardValue::Four => "4",
            CardValue::Five => "5",
            CardValue::Six => "6",
            CardValue::Seven => "7",
            CardValue::Eight => "8",
            CardValue::Nine => "9",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub enum CardVariant {
    Number(CardValue),
    Invert,
    Block,
    PlusTwo,
    PlusFour,
    Wild,
}

impl From<CardVariant> for &'static str {
    fn from(card_variant: CardVariant) -> &'static str {
        match card_variant {
            CardVariant::Number(number) => number.into(),
            CardVariant::Invert => "inverse",
            CardVariant::Block => "block",
            CardVariant::PlusTwo => "2plus",
            CardVariant::PlusFour => "4plus",
            CardVariant::Wild => "wild_card",
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct CardBundle {
    pub color: CardColor,
    pub variant: CardVariant,
}

impl CardBundle {
    pub fn texture_path(color: CardColor, variant: CardVariant) -> String {
        let color_str: &str = color.into();
        let variant_str: &str = variant.into();

        format!("cards/{}/{}_{}.png", color_str, variant_str, color_str)
    }
}
