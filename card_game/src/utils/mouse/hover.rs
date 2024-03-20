use bevy::prelude::*;

use super::{Clicked, MouseCoordinates};

#[derive(Component, Debug, Default)]
pub struct Hoverable;

#[derive(Component, Debug)]
pub struct Hovered;

pub struct HoverPlugin;

/// This plugins allows the App to know an Hoverable entity with an Image Component is being hovered by the mouse
impl Plugin for HoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (is_hovered, gizmo));
    }
}

fn gizmo(
    mut gizmos: Gizmos,
    hoverables_query: Query<(&Handle<Image>, &Transform), (With<Hovered>, Without<Clicked>)>,
    assets: Res<Assets<Image>>,
) {
    for (image, transform) in hoverables_query.iter() {
        if let Some(image) = assets.get(image) {
            let width = image.width() as f32 + 2.;
            let height = image.height() as f32 + 2.;

            gizmos.rect_2d(
                transform.translation.truncate(),
                transform.rotation.z,
                Vec2::new(width, height),
                Color::GREEN,
            );
        }
    }
}

fn is_hovered(
    hoverables_query: Query<
        (Entity, &Handle<Image>, &Transform),
        (With<Hoverable>, Without<Clicked>),
    >,
    assets: Res<Assets<Image>>,
    mouse: Res<MouseCoordinates>,
    mut commands: Commands,
) {
    for (entity, image, transform) in hoverables_query.iter() {
        let image = assets.get(image).unwrap();
        let half_width = image.width() as f32 / 2.;
        let half_height = image.height() as f32 / 2.;

        let min_x = transform.translation.x - half_width;
        let max_x = transform.translation.x + half_width;
        let min_y = transform.translation.y - half_height;
        let max_y = transform.translation.y + half_height;

        if mouse.0.x >= min_x && mouse.0.x <= max_x && mouse.0.y >= min_y && mouse.0.y <= max_y {
            commands.entity(entity).insert(Hovered);
        } else {
            commands.entity(entity).remove::<Hovered>();
        }
    }
}

#[cfg(test)] // This attribute ensures this module is only compiled when running tests
mod hover {
    // Import the testing module
    use super::*;
    // mod is_hover {
    //     use bevy::{asset::io::AssetSources, tasks::IoTaskPool};

    //     use super::*;

    //     #[test]
    //     // Hoverable [V] Hovering [V]
    //     fn hoverable_hovering() {
    //         // Setup app
    //         let mut app = App::new();
    //         app.add_plugins(AssetPlugin::default());

    //         app.init_resource::<MouseCoordinates>();
    //         app.init_resource::<Assets<Image>>();

    //         app.init_asset::<bevy::render::texture::Image>();

    //         let mut coordinates = app.world.resource_mut::<MouseCoordinates>();
    //         coordinates.0 = Vec2::new(0., 0.);

    //         // Access the asset server
    //         let asset = app.world.get_resource::<Asset<Image>>().unwrap();

    //         let image: Handle<Image> = asset_server.load("cards/card_back/card_back.png");

    //         // Add Clickable entity that is also Hovered
    //         let entity_id = app
    //             .world
    //             .spawn((Hoverable, image, Transform::from_xyz(0., 0., 0.)))
    //             .id();

    //         // Add our system
    //         app.add_systems(Update, is_hovered);

    //         // update the game once to run the system
    //         app.update();

    //         // retrieve entity after update
    //         let entity = app.world.get_entity(entity_id);

    //         assert!(entity.is_some());
    //         assert!(entity.unwrap().contains::<Hovered>());
    //     }
    // }
}
