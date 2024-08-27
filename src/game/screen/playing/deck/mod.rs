pub(crate) mod generator;

use bevy::prelude::*;

pub(super) use generator::DeckGenerator;

use crate::{
    game::{card::Card, screen::Screen},
    utils::mouse::{coordinates::UIMouseCoordinates, hover::Hovered},
};

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Deck(Vec<Card>);

impl Default for Deck {
    fn default() -> Self {
        Self(DeckGenerator::default().generate_deck())
    }
}

#[derive(Component)]
struct InDeckMarker;

#[derive(Component)]
pub(crate) struct NodeDeckMarker;

#[derive(Component)]
pub(crate) struct TextDeckMarker;

#[derive(Component)]
pub(crate) struct DeckMarker;

const DEFAULT_OFFSET: f32 = 15.;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            show_deck_data.run_if(is_deck_hovered),
            hide_deck_data.run_if(not(is_deck_hovered)),
        )
            .run_if(in_state(Screen::Playing)),
    );
}

fn is_deck_hovered(deck_hovered_query: Query<(), (With<DeckMarker>, With<Hovered>)>) -> bool {
    deck_hovered_query.get_single().is_ok()
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
mod tests {
    use super::*;

    mod is_deck_hovered {
        use super::*;

        #[derive(Resource, Default, Debug)]
        struct Test(u8);

        #[test]
        fn deck_hovered() {
            let mut app = App::new();

            app.world_mut().spawn((Hovered, DeckMarker));

            app.add_systems(
                Update,
                (|mut test_resource: ResMut<Test>| test_resource.0 += 1).run_if(is_deck_hovered),
            )
            .init_resource::<Test>();

            app.update();

            let Test(value) = app.world().resource::<Test>();

            assert_eq!(*value, 1);
        }

        #[test]
        fn deck_not_hovered() {
            let mut app = App::new();

            app.world_mut().spawn(DeckMarker);

            app.add_systems(
                Update,
                (|mut test_resource: ResMut<Test>| test_resource.0 += 1).run_if(is_deck_hovered),
            )
            .init_resource::<Test>();

            app.update();

            let Test(value) = app.world().resource::<Test>();

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
                .world_mut()
                .spawn((
                    NodeBundle {
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    NodeDeckMarker,
                ))
                .id();
            let text = app
                .world_mut()
                .spawn((TextBundle::default(), TextDeckMarker))
                .id();
            app.world_mut().entity_mut(node).add_child(text);

            app.update();

            let text = app
                .world()
                .entity(text)
                .get::<Text>()
                .and_then(|text| text.sections.first().map(|section| section.value.clone()))
                .unwrap();
            let visibility = app.world().entity(node).get::<Visibility>().unwrap();
            let style = app.world().entity(node).get::<Style>().unwrap();

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
                .world_mut()
                .spawn((
                    NodeBundle {
                        visibility: Visibility::Visible,
                        ..default()
                    },
                    NodeDeckMarker,
                ))
                .id();

            app.update();

            let visibility = app.world().entity(node).get::<Visibility>().unwrap();

            assert_eq!(visibility, Visibility::Hidden);
        }
    }
}
