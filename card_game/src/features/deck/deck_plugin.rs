use bevy::prelude::*;

use crate::{utils::mouse::hover::Hoverable, CardBundle};

use super::Deck;

pub struct DeckPlugin;

#[derive(Component)]
struct InDeckMarker;

#[derive(Component)]
struct DeckMarker;

const CARD_BACK_PATH: &str = "cards/card_back/card_back.png";

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (fill_deck, spawn_deck_sprite))
            .add_systems(Update, how_many);
    }
}

// Spawn one entity invisible per card in the deck
//TODO: change this to just be a vector of card that allows drawing.
fn fill_deck(mut commands: Commands) {
    let deck = Deck::default();

    deck.0.into_iter().for_each(|(color, variant)| {
        commands.spawn((CardBundle { color, variant }, InDeckMarker));
    });
}

fn spawn_deck_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load(CARD_BACK_PATH);

    // Text used to show how many cards are in the deck
    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![TextSection::default()],
                ..default()
            },
            transform: Transform::from_xyz(-500., 300., 0.),
            visibility: Visibility::Hidden,
            ..default()
        },
        DeckMarker,
    ));

    // Card back supposed to represent the deck
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 300., 0.),
            texture,
            ..default()
        },
        DeckMarker,
        Hoverable,
    ));
}

fn how_many(deck_query: Query<Entity, Added<InDeckMarker>>) {
    let len = deck_query.iter().count();
    if len > 0 {
        bevy::log::info!("deck size: {len}");
    }
}

//TODO create hoverable component that allows to know when hovered
// fn how_many_hover(
//     interactions: Query<&Interaction, With<DeckMarker>>,
//     mut text_query: Query<(&mut Text, &mut Visibility), With<DeckMarker>>,
// ) {
//     let (mut text, mut visibility) = text_query.single_mut();
//     interactions
//         .iter()
//         .for_each(|interaction| match interaction {
//             Interaction::None => *visibility = Visibility::Hidden,
//             _ => {
//                 *visibility = Visibility::Visible;
//                 text.sections[0].value = "aaa".to_string();
//                 bevy::log::info!("interacting");
//             }
//         })
// }
