use bevy::prelude::*;

#[cfg(feature = "dev")]
use crate::dev_tools::DevState;

use super::{click::Clicked, coordinates::MouseCoordinates};
use crate::utils::image_scaling::ScaledSize;

#[derive(Component, Debug, Default)]
pub struct Hoverable;

#[derive(Component, Debug)]
pub struct Hovered;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, is_hovered);
    #[cfg(feature = "dev")]
    app.add_systems(Update, gizmo.run_if(in_state(DevState::On)));
}

#[cfg(feature = "dev")]
fn gizmo(
    mut gizmos: Gizmos,
    hoverables_query: Query<(&ScaledSize, &Transform), (With<Hovered>, Without<Clicked>)>,
) {
    use bevy::color::palettes::css;

    for (scaled_size, transform) in hoverables_query.iter() {
        let width = scaled_size.width() + 2.;
        let height = scaled_size.height() + 2.;

        gizmos.rect_2d(
            transform.translation.truncate(),
            transform.rotation.z,
            Vec2::new(width, height),
            css::GREEN,
        );
    }
}

fn is_hovered(
    hoverables_query: Query<(Entity, &ScaledSize, &Transform), (With<Hoverable>, Without<Clicked>)>,
    mouse: Res<MouseCoordinates>,
    mut commands: Commands,
) {
    for (entity, scaled_size, transform) in hoverables_query.iter() {
        let half_width = scaled_size.width() / 2.;
        let half_height = scaled_size.height() / 2.;

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
    use crate::utils::test;

    mod is_hovered {
        use super::*;
        use test::asset_loading::{check_loaded, is_asset_loaded, TestAssetLoadingState};

        use crate::{
            entities::cards::CARD_BACK_PATH,
            utils::{image_scaling, mouse::coordinates::MouseCoordinates},
        };

        #[test]
        // Hoverable [V] Hovering [V]
        fn hoverable_hovering() {
            // Setup app
            let mut app = App::new();
            app.add_plugins((MinimalPlugins, test::plugin, image_scaling::plugin))
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
                .spawn((
                    Hoverable,
                    image,
                    ScaledSize::default(),
                    Transform::from_xyz(0., 0., 0.),
                ))
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
            app.add_plugins((MinimalPlugins, test::plugin, image_scaling::plugin))
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
                .spawn((
                    Hoverable,
                    image,
                    ScaledSize::default(),
                    Transform::from_xyz(0., 0., 0.),
                ))
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
            app.add_plugins((MinimalPlugins, test::plugin, image_scaling::plugin))
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
                .spawn((
                    image,
                    ScaledSize::default(),
                    Transform::from_xyz(0., 0., 0.),
                ))
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
            app.add_plugins((MinimalPlugins, test::plugin, image_scaling::plugin))
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
                .spawn((
                    image,
                    ScaledSize::default(),
                    Transform::from_xyz(0., 0., 0.),
                ))
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
