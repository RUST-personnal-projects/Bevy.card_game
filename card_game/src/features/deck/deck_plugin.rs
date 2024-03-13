use bevy::prelude::*;

use crate::CardBundle;

use super::Deck;

pub struct DeckPlugin;

#[derive(Component)]
struct InDeckMarker;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, how_many);
    }
}

fn startup(mut commands: Commands) {
    let deck = Deck::default();

    deck.0.into_iter().for_each(|(color, variant)| {
        commands.spawn((CardBundle { color, variant }, InDeckMarker));
    });
}

fn how_many(deck_query: Query<Entity, Added<InDeckMarker>>) {
    let len = deck_query.iter().count();
    if len > 0 {
        bevy::log::info!("deck size: {len}");
    }
}
