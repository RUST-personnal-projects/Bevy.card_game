use bevy::{color::palettes::css, prelude::*};

use crate::utils::Loaded;

use super::{Clicked, MouseCoordinates};

#[derive(Component, Debug, Default)]
pub struct Hoverable;

#[derive(Component, Debug)]
pub struct Hovered;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (is_hovered, gizmo));
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
                css::GREEN,
            );
        }
    }
}

fn is_hovered(
    hoverables_query: Query<
        (Entity, &Handle<Image>, &Transform),
        (With<Hoverable>, With<Loaded<Image>>, Without<Clicked>),
    >,
    images: Res<Assets<Image>>,
    mouse: Res<MouseCoordinates>,
    mut commands: Commands,
) {
    for (entity, image, transform) in hoverables_query.iter() {
        let image = images.get(image).unwrap();
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
mod tests {
    use super::*;
    use crate::utils::{
        test,
        test::{check_loaded, TestAssetLoadingState},
    };

    mod is_hovered {
        use crate::{features::CARD_BACK_PATH, utils::assets::is_asset_loaded};

        use super::*;

        #[test]
        // Hoverable [V] Hovering [V]
        fn hoverable_hovering() {
            // Setup app
            let mut app = App::new();
            app.add_plugins((MinimalPlugins, test::plugin))
                .init_resource::<MouseCoordinates>();

            // Add mouse coordinates Resource
            let mut coordinates = app.world_mut().resource_mut::<MouseCoordinates>();
            coordinates.0 = Vec2::new(0., 0.);

            // Access the asset server and start loading Image
            let asset_server = app.world_mut().resource_mut::<AssetServer>();

            let image: Handle<Image> = asset_server.load(CARD_BACK_PATH);

            // Add Hoverable entity that is Hovered
            let entity_id = app
                .world_mut()
                .spawn((Hoverable, image, Transform::from_xyz(0., 0., 0.)))
                .id();

            // Add two systems: one is a test system that checks asset is loaded, second is checking if Image asset is hovered
            app.add_systems(
                Update,
                (
                    is_asset_loaded::<Image>,
                    check_loaded::<Image>,
                    is_hovered.run_if(in_state(TestAssetLoadingState::Loaded)),
                )
                    .chain(),
            );

            // update the game until asset is loaded then check if hovered
            while *app.world().resource::<State<TestAssetLoadingState>>().get()
                == TestAssetLoadingState::Loading
            {
                app.update();
            }

            // retrieve entity after update
            let entity = app.world().get_entity(entity_id);

            assert!(entity.is_some());
            assert!(entity.unwrap().contains::<Hovered>());
        }

        #[test]
        // Hoverable [V] Hovering [X]
        fn hoverable_not_hovering() {
            // Setup app
            let mut app = App::new();
            app.add_plugins((MinimalPlugins, test::plugin))
                .init_resource::<MouseCoordinates>();

            // Add mouse coordinates Resource
            let mut coordinates = app.world_mut().resource_mut::<MouseCoordinates>();
            coordinates.0 = Vec2::new(200., 0.);

            // Access the asset server and start loading Image
            let asset_server = app.world_mut().resource_mut::<AssetServer>();

            let image: Handle<Image> = asset_server.load(CARD_BACK_PATH);

            // Add Hoverable entity that is Hovered
            let entity_id = app
                .world_mut()
                .spawn((Hoverable, image, Transform::from_xyz(0., 0., 0.)))
                .id();

            // Add two systems: one is a test system that checks asset is loaded, second is checking if Image asset is hovered
            app.add_systems(
                Update,
                (
                    is_asset_loaded::<Image>,
                    check_loaded::<Image>,
                    is_hovered.run_if(in_state(TestAssetLoadingState::Loaded)),
                )
                    .chain(),
            );

            // update the game until asset is loaded then check if hovered
            while *app.world().resource::<State<TestAssetLoadingState>>().get()
                == TestAssetLoadingState::Loading
            {
                app.update();
            }

            // retrieve entity after update
            let entity = app.world().get_entity(entity_id);

            assert!(entity.is_some());
            assert!(!entity.unwrap().contains::<Hovered>());
        }

        #[test]
        // Hoverable [X] Hovering [V]
        fn not_hoverable_hovering() {
            // Setup app
            let mut app = App::new();
            app.add_plugins((MinimalPlugins, test::plugin))
                .init_resource::<MouseCoordinates>();

            // Add mouse coordinates Resource
            let mut coordinates = app.world_mut().resource_mut::<MouseCoordinates>();
            coordinates.0 = Vec2::new(0., 0.);

            // Access the asset server and start loading Image
            let asset_server = app.world_mut().resource_mut::<AssetServer>();

            let image: Handle<Image> = asset_server.load(CARD_BACK_PATH);

            // Add Hoverable entity that is Hovered
            let entity_id = app
                .world_mut()
                .spawn((image, Transform::from_xyz(0., 0., 0.)))
                .id();

            // Add two systems: one is a test system that checks asset is loaded, second is checking if Image asset is hovered
            app.add_systems(
                Update,
                (
                    is_asset_loaded::<Image>,
                    check_loaded::<Image>,
                    is_hovered.run_if(in_state(TestAssetLoadingState::Loaded)),
                )
                    .chain(),
            );

            // update the game until asset is loaded then check if hovered
            while *app.world().resource::<State<TestAssetLoadingState>>().get()
                == TestAssetLoadingState::Loading
            {
                app.update();
            }

            // retrieve entity after update
            let entity = app.world().get_entity(entity_id);

            assert!(entity.is_some());
            assert!(!entity.unwrap().contains::<Hovered>());
        }

        #[test]
        // Hoverable [X] Hovering [X]
        fn not_hoverable_not_hovering() {
            // Setup app
            let mut app = App::new();
            app.add_plugins((MinimalPlugins, test::plugin))
                .init_resource::<MouseCoordinates>();

            // Add mouse coordinates Resource
            let mut coordinates = app.world_mut().resource_mut::<MouseCoordinates>();
            coordinates.0 = Vec2::new(200., 0.);

            // Access the asset server and start loading Image
            let asset_server = app.world_mut().resource_mut::<AssetServer>();

            let image: Handle<Image> = asset_server.load(CARD_BACK_PATH);

            // Add Hoverable entity that is Hovered
            let entity_id = app
                .world_mut()
                .spawn((image, Transform::from_xyz(0., 0., 0.)))
                .id();

            // Add two systems: one is a test system that checks asset is loaded, second is checking if Image asset is hovered
            app.add_systems(
                Update,
                (
                    is_asset_loaded::<Image>,
                    check_loaded::<Image>,
                    is_hovered.run_if(in_state(TestAssetLoadingState::Loaded)),
                )
                    .chain(),
            );

            // update the game until asset is loaded then check if hovered
            while *app.world().resource::<State<TestAssetLoadingState>>().get()
                == TestAssetLoadingState::Loading
            {
                app.update();
            }

            // retrieve entity after update
            let entity = app.world().get_entity(entity_id);

            assert!(entity.is_some());
            assert!(!entity.unwrap().contains::<Hovered>());
        }
    }
}
