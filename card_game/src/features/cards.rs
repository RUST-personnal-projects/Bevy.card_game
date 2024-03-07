use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub enum CardColor {
    Yellow,
    Red,
    Blue,
    Green,
    Wild,
}

impl From<CardColor> for String {
    fn from(card_color: CardColor) -> String {
        match card_color {
            CardColor::Yellow => "yellow".to_string(),
            CardColor::Red => "red".to_string(),
            CardColor::Blue => "blue".to_string(),
            CardColor::Green => "green".to_string(),
            CardColor::Wild => "wild".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub enum CardVariant {
    Number(u8),
    Invert,
    Block,
    PlusTwo,
    PlusFour,
    Wild,
}

impl From<CardVariant> for String {
    fn from(card_variant: CardVariant) -> String {
        match card_variant {
            CardVariant::Number(number) => number.to_string(),
            CardVariant::Invert => "inverse".to_string(),
            CardVariant::Block => "block".to_string(),
            CardVariant::PlusTwo => "2plus".to_string(),
            CardVariant::PlusFour => "4plus".to_string(),
            CardVariant::Wild => "wild_card".to_string(),
        }
    }
}

#[derive(Bundle, Debug, Clone)]
pub struct CardBundle {
    pub color: CardColor,
    pub variant: CardVariant,
}

impl CardBundle {
    pub fn texture_path(color: CardColor, variant: CardVariant) -> String {
        let color_str: String = color.into();
        let variant_str: String = variant.into();

        format!("cards/{}/{}_{}.png", color_str, variant_str, color_str)
    }
}
