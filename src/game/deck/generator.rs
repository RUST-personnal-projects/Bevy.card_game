use rand::seq::SliceRandom;

use crate::game::card::{Card, CardColor, ColoredVariant, WildVariant};

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
fn add_colored_card(variant: ColoredVariant, number: u8) -> Vec<Card> {
    let mut cards = Vec::new();
    [
        CardColor::Blue,
        CardColor::Yellow,
        CardColor::Red,
        CardColor::Green,
    ]
    .into_iter()
    .for_each(|color| {
        cards.append(&mut add_card(Card::Colored(variant, color), number));
    });

    cards
}

/// Add a Wild card a given `number` of times
fn add_wild_card(variant: WildVariant, number: u8) -> Vec<Card> {
    add_card(Card::Wild(variant), number)
}

/// Add any card a given `number` of times
fn add_card(card: Card, number: u8) -> Vec<Card> {
    let mut cards = Vec::new();
    (1..=number).for_each(|_| cards.push(card));

    cards
}

impl DeckGenerator {
    /// ## Generates a deck.
    ///
    /// Each card that can be colored is created N x C times, with N being the number of times
    /// we want that card to be created and C the number of possible colors the card has.
    ///
    pub fn generate_deck(self) -> Vec<Card> {
        let mut deck = Vec::new();

        // Add all colored 0 cards
        deck.append(&mut add_colored_card(
            ColoredVariant::Number(0),
            self.number0,
        ));
        // Add all colored cards from 1 to 9
        (1..=9).for_each(|card_number| {
            deck.append(&mut add_colored_card(
                ColoredVariant::Number(card_number),
                self.numbers,
            ))
        });
        // Add all colored block cards
        deck.append(&mut add_colored_card(ColoredVariant::Block, self.block));
        // Add all colored invert cards
        deck.append(&mut add_colored_card(ColoredVariant::Invert, self.invert));
        // Add all colored +2 cards
        deck.append(&mut add_colored_card(ColoredVariant::PlusTwo, self.plus_2));
        // Add all +4 cards
        deck.append(&mut add_wild_card(WildVariant::PlusFour, self.plus_4));
        // Add all wild cards
        deck.append(&mut add_wild_card(WildVariant::ColorChange, self.wild_card));

        // shuffle the deck using rand
        deck.shuffle(&mut rand::thread_rng());
        deck
    }
}
