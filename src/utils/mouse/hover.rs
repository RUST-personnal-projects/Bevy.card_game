use std::any::TypeId;

#[cfg(feature = "dev")]
use crate::dev_tools::DevState;
use bevy::{ecs::component::ComponentId, prelude::*, ptr::PtrMut, reflect::ReflectFromPtr};
use pretty_type_name::pretty_type_name_str;

use super::{click::Clicked, coordinates::MouseCoordinates};
use crate::utils::image_scaling::ScaledSize;

#[derive(Component, Debug, Default)]
pub struct Hoverable;

#[derive(Component, Debug)]
pub struct Hovered;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, is_hovered);
    #[cfg(feature = "dev")]
    app.add_systems(Update, (gizmo, entity_info).run_if(in_state(DevState::On)));
}

#[cfg(feature = "dev")]
fn gizmo(
    mut gizmos: Gizmos,
    hoverables_query: Query<(&ScaledSize, &GlobalTransform), (With<Hovered>, Without<Clicked>)>,
) {
    use bevy::color::palettes::css;

    for (scaled_size, transform) in hoverables_query.iter() {
        let width = scaled_size.width() + 2.;
        let height = scaled_size.height() + 2.;

        let (_, _, translation) = transform.to_scale_rotation_translation();

        gizmos.rect_2d(
            Isometry2d::from_translation(translation.truncate()),
            Vec2::new(width, height),
            css::GREEN,
        );
    }
}

fn is_hovered(
    hoverables_query: Query<
        (Entity, &ScaledSize, &GlobalTransform),
        (With<Hoverable>, Without<Clicked>),
    >,
    mouse: Res<MouseCoordinates>,
    mut commands: Commands,
) {
    for (entity, scaled_size, transform) in hoverables_query.iter() {
        let half_width = scaled_size.width() / 2.;
        let half_height = scaled_size.height() / 2.;

        let translation = transform.translation();

        let min_x = translation.x - half_width;
        let max_x = translation.x + half_width;
        let min_y = translation.y - half_height;
        let max_y = translation.y + half_height;

        if mouse.0.x >= min_x && mouse.0.x <= max_x && mouse.0.y >= min_y && mouse.0.y <= max_y {
            commands.entity(entity).insert(Hovered);
        } else {
            commands.entity(entity).remove::<Hovered>();
        }
    }
}

/// Credits go to [bevy_inspector_egui](https://crates.io/crates/bevy_inspector_egui) for the general idea how to use reflection to retrieve components
#[cfg(feature = "dev")]
fn entity_info(world: &mut World) {
    let entities = world
        .query_filtered::<Entity, Added<Hovered>>()
        .iter(world)
        .collect::<Vec<_>>();

    for entity in entities {
        if let Ok(entity_ref) = world.get_entity(entity) {
            let components = entity_components(entity_ref, world);
            let mut components_logs = Vec::new();
            let mut components_names = Vec::new();
            for (component_id, component_type_id, name) in components {
                components_names.push(name.clone());
                if let Some(value) =
                    component_reflected_value(entity_ref, world, component_id, component_type_id)
                {
                    components_logs.push(format!("{}: {}", name, log_reflect(value)));
                }
            }
            info!(
                "\n{}\n{}",
                components_names.join(", "),
                components_logs.join("\n")
            );
        }
    }
}

/// Retrieves bevy [`ComponentId`] and std [`TypeId`] for every components attached to an entity
#[cfg(feature = "dev")]
fn entity_components(
    entity_ref: EntityRef,
    world: &World,
) -> Vec<(ComponentId, Option<TypeId>, String)> {
    entity_ref
        .archetype()
        .components()
        .map(|component_id| {
            let info = world.components().get_info(component_id).unwrap();

            (
                component_id,
                info.type_id(),
                pretty_type_name_str(info.name()),
            )
        })
        .collect::<Vec<_>>()
}

/// Retrieves a component [`PtrMut`] and convert it to a [`Reflect`]
#[cfg(feature = "dev")]
fn component_reflected_value<'a>(
    entity_ref: EntityRef,
    world: &World,
    component_id: ComponentId,
    component_type_id: Option<TypeId>,
) -> Option<&'a mut dyn Reflect> {
    let value = entity_ref.get_by_id(component_id).ok()?;
    let type_id = component_type_id?;
    let type_registry = world.resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    let registration = type_registry.get(type_id)?;
    let reflect_from_ptr = registration.data::<ReflectFromPtr>()?;

    let ptr: PtrMut<'a> = unsafe { PtrMut::new(std::ptr::NonNull::new(value.as_ptr())?) };
    // As stated in as_reflect_mut we need to ensure that the Ptr can be converted to something reflected by checking that they would have same [`TypeId`]
    reflect_from_ptr
        .type_id()
        .eq(&type_id)
        .then(|| unsafe { reflect_from_ptr.as_reflect_mut(ptr) })
}

/// Recursively retrieves any type of [`Reflect`] down to [`bevy::reflect::ReflectMut::Value`]
#[cfg(feature = "dev")]
fn log_reflect(value: &dyn PartialReflect) -> String {
    match value.reflect_ref() {
        bevy::reflect::ReflectRef::Struct(value) => {
            let mut fields = Vec::new();
            for i in 0..value.field_len() {
                let field = log_reflect(value.field_at(i).unwrap());
                let name = value.name_at(i).unwrap();
                fields.push(format!("{}: {}", name, field,));
            }
            fields.join(", ")
        }
        bevy::reflect::ReflectRef::TupleStruct(value) => {
            let mut fields = Vec::new();
            for i in 0..value.field_len() {
                fields.push(log_reflect(value.field(i).unwrap()));
            }
            format!("({})", fields.join(", "))
        }
        bevy::reflect::ReflectRef::Tuple(value) => {
            let mut fields = Vec::new();
            for i in 0..value.field_len() {
                fields.push(log_reflect(value.field(i).unwrap()));
            }
            format!("({})", fields.join(", "))
        }
        bevy::reflect::ReflectRef::List(value) => {
            let mut fields = Vec::new();
            for i in 0..value.len() {
                fields.push(log_reflect(value.get(i).unwrap()));
            }
            format!("[{}]", fields.join(", "))
        }
        bevy::reflect::ReflectRef::Array(value) => {
            let mut fields = Vec::new();
            for i in 0..value.len() {
                fields.push(log_reflect(value.get(i).unwrap()));
            }
            format!("[{}]", fields.join(", "))
        }
        bevy::reflect::ReflectRef::Map(value) => {
            let mut fields = Vec::new();
            for i in 0..value.len() {
                let (key, value) = value.get_at(i).unwrap();
                fields.push(format!("{:?}: {}", key, log_reflect(value)));
            }
            format!("[{}]", fields.join(", "))
        }
        bevy::reflect::ReflectRef::Enum(value) => match value.variant_type() {
            bevy::reflect::VariantType::Struct => {
                let mut fields = Vec::new();
                for i in 0..value.field_len() {
                    let field = log_reflect(value.field_at(i).unwrap());
                    let name = value.name_at(i).unwrap();
                    fields.push(format!("{}: {}", name, field));
                }
                format!("{}: {}", value.variant_name(), fields.join(", "))
            }
            bevy::reflect::VariantType::Tuple => {
                let mut fields = Vec::new();
                for i in 0..value.field_len() {
                    fields.push(log_reflect(value.field_at(i).unwrap()));
                }
                format!("({})", fields.join(", "))
            }
            bevy::reflect::VariantType::Unit => value.variant_name().to_string(),
        },
        bevy::reflect::ReflectRef::Set(value) => {
            let mut fields = Vec::new();
            for field in value.iter() {
                fields.push(log_reflect(field));
            }
            format!("[{}]", fields.join(", "))
        }
        bevy::reflect::ReflectRef::Opaque(value) => format!("{:?}", value),
    }
}

#[cfg(test)] // This attribute ensures this module is only compiled when running tests
mod tests {
    use super::*;
    use crate::utils::test;

    mod is_hovered {
        use super::*;
        use test::asset_loading::{are_all_images_loaded, is_image_loaded, TestAssetLoadingState};

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
                    Sprite::from_image(image),
                    ScaledSize::default(),
                    Transform::from_xyz(0., 0., 0.),
                ))
                .id();

            // Add two systems: one is a test system that checks asset is loaded, second is checking if Image asset is hovered
            app.add_systems(
                Update,
                (
                    are_all_images_loaded,
                    is_image_loaded,
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

            assert!(entity.is_ok());
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
                    Sprite::from_image(image),
                    ScaledSize::default(),
                    Transform::from_xyz(0., 0., 0.),
                ))
                .id();

            // Add two systems: one is a test system that checks asset is loaded, second is checking if Image asset is hovered
            app.add_systems(
                Update,
                (
                    are_all_images_loaded,
                    is_image_loaded,
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

            assert!(entity.is_ok());
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
                    Sprite::from_image(image),
                    ScaledSize::default(),
                    Transform::from_xyz(0., 0., 0.),
                ))
                .id();

            // Add two systems: one is a test system that checks asset is loaded, second is checking if Image asset is hovered
            app.add_systems(
                Update,
                (
                    are_all_images_loaded,
                    is_image_loaded,
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

            assert!(entity.is_ok());
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
                    Sprite::from_image(image.clone_weak()),
                    ScaledSize::default(),
                    Transform::from_xyz(0., 0., 0.),
                ))
                .id();

            // Add two systems: one is a test system that checks asset is loaded, second is checking if Image asset is hovered
            app.add_systems(
                Update,
                (
                    are_all_images_loaded,
                    is_image_loaded,
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

            assert!(entity.is_ok());
            assert!(!entity.unwrap().contains::<Hovered>());
        }
    }
}
