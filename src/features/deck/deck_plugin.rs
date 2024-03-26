use bevy::prelude::*;

use crate::{
    utils::mouse::{coordinates::UIMouseCoordinates, hover::Hoverable, Clickable, Hovered},
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

const DEFAULT_OFFSET: f32 = 15.;

pub const CARD_BACK_PATH: &str = "cards/card_back/card_back.png";

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (fill_deck, spawn_deck_sprite))
            .add_systems(
                Update,
                (
                    show_deck_data.run_if(is_deck_hovered),
                    hide_deck_data.run_if(not(is_deck_hovered)),
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

/// Spawn a card back sprite representing the deck and an UI node containing text to show how many cards are left
fn spawn_deck_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI node
    commands
        .spawn((
            NodeBundle {
                background_color: BackgroundColor(Color::DARK_GRAY),
                border_color: BorderColor(Color::BLACK),
                visibility: Visibility::Hidden,
                ..default()
            },
            NodeDeckMarker,
        ))
        .with_children(|builder| {
            builder.spawn((TextBundle::default(), TextDeckMarker));
        });

    let texture = asset_server.load(CARD_BACK_PATH);
    // Card back
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

fn is_deck_hovered(deck_hovered_query: Query<(), (With<DeckMarker>, With<Hovered>)>) -> bool {
    deck_hovered_query.iter().count() == 1
}

/// Make the node showing how many cards left in deck and update it's style position, update text inside node
fn show_deck_data(
    mut node_query: Query<(&mut Visibility, &mut Style), With<NodeDeckMarker>>,
    mut text_query: Query<&mut Text, With<TextDeckMarker>>,
    ui_mouse_coordinates: Res<UIMouseCoordinates>,
    deck_query: Query<(), With<InDeckMarker>>,
) {
    let (mut visibility, mut style) = node_query.single_mut();
    let mut text = text_query.single_mut();

    let len = deck_query.iter().count();

    let UIMouseCoordinates(Vec2 { x, y }) = ui_mouse_coordinates.into_inner();
    style.left = Val::Px(*x + DEFAULT_OFFSET);
    style.top = Val::Px(*y);

    *text = Text::from_section(format!("cards remaining: {}", len), TextStyle::default());
    *visibility = Visibility::Visible;
}

/// Hides the node showing how many cards left in deck
fn hide_deck_data(mut node_query: Query<&mut Visibility, With<NodeDeckMarker>>) {
    let mut visibility = node_query.single_mut();

    *visibility = Visibility::Hidden;
}

#[cfg(test)]
mod deck_plugin {
    use super::*;
    use crate::utils::test::count_entities::*;

    mod fill_deck {
        use super::*;

        #[test]
        fn spawned_all_entities() {
            let deck_size = Deck::default().0.len();

            let mut app = App::new();

            app.add_systems(Startup, fill_deck)
                .add_systems(Update, count_entities::<InDeckMarker>)
                .init_resource::<EntityCount>();

            app.update();

            let entities_count = app.world.resource::<EntityCount>();

            assert_eq!(entities_count.0, deck_size);
        }
    }

    mod is_deck_hovered {
        use super::*;

        #[derive(Resource, Default, Debug)]
        struct Test(u8);

        #[test]
        fn deck_hovered() {
            let mut app = App::new();

            app.world.spawn((Hovered, DeckMarker));

            app.add_systems(
                Update,
                (|mut test_resource: ResMut<Test>| test_resource.0 += 1).run_if(is_deck_hovered),
            )
            .init_resource::<Test>();

            app.update();

            let Test(value) = app.world.resource::<Test>();

            assert_eq!(*value, 1);
        }

        #[test]
        fn deck_not_hovered() {
            let mut app = App::new();

            app.world.spawn(DeckMarker);

            app.add_systems(
                Update,
                (|mut test_resource: ResMut<Test>| test_resource.0 += 1).run_if(is_deck_hovered),
            )
            .init_resource::<Test>();

            app.update();

            let Test(value) = app.world.resource::<Test>();

            assert_eq!(*value, 0);
        }
    }

    mod show_deck_data {
        use super::*;

        #[test]
        fn set_visibility_style_and_text() {
            let mut app = App::new();

            app.add_systems(Update, show_deck_data)
                .init_resource::<UIMouseCoordinates>();

            let node = app
                .world
                .spawn((
                    NodeBundle {
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    NodeDeckMarker,
                ))
                .id();
            let text = app
                .world
                .spawn((TextBundle::default(), TextDeckMarker))
                .id();
            app.world.entity_mut(node).add_child(text);

            app.update();

            let text = app
                .world
                .entity(text)
                .get::<Text>()
                .and_then(|text| text.sections.first().map(|section| section.value.clone()))
                .unwrap();
            let visibility = app.world.entity(node).get::<Visibility>().unwrap().clone();
            let style = app.world.entity(node).get::<Style>().unwrap().clone();

            assert_eq!(visibility, Visibility::Visible);
            assert_eq!(text, "cards remaining: 0".to_string());
            assert_eq!(
                (style.top, style.left),
                (Val::Px(0.), Val::Px(DEFAULT_OFFSET))
            );
        }
    }

    mod hide_deck_data {
        use super::*;

        #[test]
        fn set_visibility() {
            let mut app = App::new();

            app.add_systems(Update, hide_deck_data);

            let node = app
                .world
                .spawn((
                    NodeBundle {
                        visibility: Visibility::Visible,
                        ..default()
                    },
                    NodeDeckMarker,
                ))
                .id();

            app.update();

            let visibility = app.world.entity(node).get::<Visibility>().unwrap().clone();

            assert_eq!(visibility, Visibility::Hidden);
        }
    }
}
