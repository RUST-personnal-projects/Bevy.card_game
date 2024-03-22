use bevy::prelude::*;

use crate::{
    utils::mouse::{hover::Hoverable, Clickable, Hovered},
    CardBundle,
};

use super::Deck;

pub struct DeckPlugin;

#[derive(Component)]
struct InDeckMarker;

#[derive(Component)]
struct NodeDeckMarker;

#[derive(Component)]
struct TextDeckMarker;

#[derive(Component)]
struct DeckMarker;

pub const CARD_BACK_PATH: &str = "cards/card_back/card_back.png";

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (fill_deck, spawn_deck_sprite))
            .add_systems(
                Update,
                (
                    how_many_in_deck.run_if(run_if_deck_hovered),
                    hide_node.run_if(not(run_if_deck_hovered)),
                ),
            );
    }
}

// Spawn one entity invisible per card in the deck
fn fill_deck(mut commands: Commands) {
    let deck = Deck::default();

    deck.0.into_iter().for_each(|(color, variant)| {
        commands.spawn((CardBundle { color, variant }, InDeckMarker));
    });
}

fn spawn_deck_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load(CARD_BACK_PATH);

    // Text used to show how many cards are in the deck
    commands
        .spawn((
            NodeBundle {
                // style: Style {
                //     align_self: AlignSelf::End,
                //     ..default()
                // },
                background_color: BackgroundColor(Color::DARK_GRAY),
                border_color: BorderColor(Color::BLACK),
                visibility: Visibility::Hidden,
                ..default()
            },
            NodeDeckMarker,
        ))
        .with_children(|builder| {
            builder.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::default()],
                        ..default()
                    },
                    ..default()
                },
                TextDeckMarker,
            ));
        });

    // Card back supposed to represent the deck
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 300., 0.),
            texture,
            ..default()
        },
        DeckMarker,
        Hoverable,
        Clickable,
    ));
}

fn run_if_deck_hovered(deck_hovered_query: Query<(), (With<DeckMarker>, With<Hovered>)>) -> bool {
    deck_hovered_query.iter().count() == 1
}

fn how_many_in_deck(
    mut node_query: Query<(&mut Visibility, &mut Style), With<NodeDeckMarker>>,
    mut text_query: Query<&mut Text, With<TextDeckMarker>>,
    window_query: Query<&Window>,
    deck_query: Query<(), With<InDeckMarker>>,
) {
    let (mut visibility, mut style) = node_query.single_mut();
    let mut text = text_query.single_mut();
    let window = window_query.single();
    let len = deck_query.iter().count();

    if let Some(cursor) = window.cursor_position() {
        style.left = Val::Px(cursor.x + 15.);
        style.top = Val::Px(cursor.y);
    }

    *text = Text::from_section(format!("cards remaining: {}", len), TextStyle::default());
    *visibility = Visibility::Visible;
}

fn hide_node(mut node_query: Query<&mut Visibility, With<NodeDeckMarker>>) {
    let mut visibility = node_query.single_mut();

    *visibility = Visibility::Hidden;
}
