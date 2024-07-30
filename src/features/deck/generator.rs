use rand::seq::SliceRandom;

use crate::features::cards::{CardColor, CardVariant};

use super::CardInfo;

pub struct DeckGenerator {
    number0: u8,
    numbers: u8,
    invert: u8,
    block: u8,
    plus_2: u8,
    plus_4: u8,
    wild_card: u8,
}

impl Default for DeckGenerator {
    fn default() -> Self {
        Self {
            number0: 1,
            numbers: 2,
            invert: 2,
            block: 2,
            plus_2: 2,
            plus_4: 4,
            wild_card: 4,
        }
    }
}

/// Add all colored variants of a numbered card a given `number` of times
fn add_colored_card(variant: CardVariant, number: u8) -> Vec<CardInfo> {
    let mut cards = Vec::new();
    [
        CardColor::Blue,
        CardColor::Yellow,
        CardColor::Red,
        CardColor::Green,
    ]
    .into_iter()
    .for_each(|color| {
        cards.append(&mut add_card(variant, color, number));
    });

    cards
}

/// Add a Wild card a given `number` of times
fn add_wild_card(variant: CardVariant, number: u8) -> Vec<CardInfo> {
    add_card(variant, CardColor::Wild, number)
}

/// Add any card a given `number` of times
fn add_card(variant: CardVariant, color: CardColor, number: u8) -> Vec<CardInfo> {
    let mut cards = Vec::new();
    (1..=number).for_each(|_| cards.push((color, variant)));

    cards
}

impl DeckGenerator {
    /// ## Generates a deck.
    ///
    /// Each card that can be colored is created N x C times, with N being the number of times
    /// we want that card to be created and C the number of possible colors the card has.
    ///
    pub fn generate_deck(self) -> Vec<CardInfo> {
        let mut deck = Vec::new();

        // Add all colored 0 cards
        deck.append(&mut add_colored_card(CardVariant::Number(0), self.number0));
        // Add all colored cards from 1 to 9
        (1..=9).for_each(|card_number| {
            deck.append(&mut add_colored_card(
                CardVariant::Number(card_number),
                self.numbers,
            ))
        });
        // Add all colored block cards
        deck.append(&mut add_colored_card(CardVariant::Block, self.block));
        // Add all colored invert cards
        deck.append(&mut add_colored_card(CardVariant::Invert, self.invert));
        // Add all colored +2 cards
        deck.append(&mut add_colored_card(CardVariant::PlusTwo, self.plus_2));
        // Add all +4 cards
        deck.append(&mut add_wild_card(CardVariant::PlusFour, self.plus_4));
        // Add all wild cards
        deck.append(&mut add_wild_card(CardVariant::Wild, self.wild_card));

        // shuffle the deck using rand
        deck.shuffle(&mut rand::thread_rng());
        deck
    }
}
