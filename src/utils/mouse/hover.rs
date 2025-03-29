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

    mod is_hovered {
        use super::*;
        use bevy::state::app::StatesPlugin;
        use bevy_asset_loader::prelude::*;

        use crate::utils::{image_scaling, mouse::coordinates::MouseCoordinates};

        #[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
        pub enum TestStates {
            #[default]
            Loading,
            Loaded,
        }

        #[derive(AssetCollection, Resource)]
        pub struct TestImageAssets {
            // Special cards:
            #[asset(path = "images/cards/card_back.png")]
            pub card_back: Handle<Image>,
        }

        fn timeout(time: Res<Time>) {
            if time.elapsed_secs_f64() > 10. {
                panic!("The asset loader did not change the state in 10 seconds");
            }
        }

        struct TestPlugin(Vec2);

        impl Plugin for TestPlugin {
            fn build(&self, app: &mut App) {
                app.add_plugins((
                    MinimalPlugins,
                    AssetPlugin::default(),
                    ImagePlugin::default(),
                    StatesPlugin,
                    image_scaling::plugin,
                ));
                app.init_state::<TestStates>();

                app.add_loading_state(
                    LoadingState::new(TestStates::Loading)
                        .continue_to_state(TestStates::Loaded)
                        .load_collection::<TestImageAssets>(),
                );

                app.insert_resource(MouseCoordinates(self.0));

                app.add_systems(Update, timeout.run_if(in_state(TestStates::Loading)));
            }
        }

        #[test]
        fn hoverable_hovering() {
            let mut app = App::new();

            app.add_plugins(TestPlugin(Vec2::new(0., 0.)));

            app.add_systems(
                OnEnter(TestStates::Loaded),
                (
                    |images: Res<TestImageAssets>, mut commands: Commands| {
                        commands.spawn((
                            Hoverable,
                            Sprite::from_image(images.card_back.clone_weak()),
                            ScaledSize::default(),
                            Transform::from_xyz(0., 0., 0.),
                        ));
                    },
                    is_hovered,
                    |entity_query: Query<(), With<Hovered>>, mut exit: EventWriter<AppExit>| {
                        assert!(entity_query.get_single().is_ok());
                        exit.send(AppExit::Success);
                    },
                )
                    .chain(),
            );

            app.run();
        }

        #[test]
        fn hoverable_hovering_two() {
            let mut app = App::new();

            app.add_plugins(TestPlugin(Vec2::new(0., 0.)));

            app.add_systems(
                OnEnter(TestStates::Loaded),
                (
                    |images: Res<TestImageAssets>, mut commands: Commands| {
                        commands.spawn((
                            Hoverable,
                            Sprite::from_image(images.card_back.clone_weak()),
                            ScaledSize::default(),
                            Transform::from_xyz(0., 0., 0.),
                        ));
                        commands.spawn((
                            Hoverable,
                            Sprite::from_image(images.card_back.clone_weak()),
                            ScaledSize::default(),
                            Transform::from_xyz(0., 0., 0.),
                        ));
                    },
                    is_hovered,
                    |entity_query: Query<(), With<Hovered>>, mut exit: EventWriter<AppExit>| {
                        assert!(entity_query.iter().len() == 2);
                        exit.send(AppExit::Success);
                    },
                )
                    .chain(),
            );

            app.run();
        }

        #[test]
        fn hoverable_not_hovering() {
            let mut app = App::new();

            app.add_plugins(TestPlugin(Vec2::new(2000., 0.)));

            app.add_systems(
                OnEnter(TestStates::Loaded),
                (
                    |images: Res<TestImageAssets>, mut commands: Commands| {
                        commands.spawn((
                            Hoverable,
                            Sprite::from_image(images.card_back.clone_weak()),
                            ScaledSize::default(),
                            Transform::from_xyz(0., 0., 0.),
                        ));
                    },
                    is_hovered,
                    |entity_query: Query<(), With<Hovered>>, mut exit: EventWriter<AppExit>| {
                        assert!(entity_query.is_empty());
                        exit.send(AppExit::Success);
                    },
                )
                    .chain(),
            );

            app.run();
        }

        #[test]
        fn not_hoverable_hovering() {
            let mut app = App::new();

            app.add_plugins(TestPlugin(Vec2::new(0., 0.)));

            app.add_systems(
                OnEnter(TestStates::Loaded),
                (
                    |images: Res<TestImageAssets>, mut commands: Commands| {
                        commands.spawn((
                            Sprite::from_image(images.card_back.clone_weak()),
                            ScaledSize::default(),
                            Transform::from_xyz(0., 0., 0.),
                        ));
                    },
                    is_hovered,
                    |entity_query: Query<(), With<Hovered>>, mut exit: EventWriter<AppExit>| {
                        assert!(entity_query.is_empty());
                        exit.send(AppExit::Success);
                    },
                )
                    .chain(),
            );
        }

        #[test]
        fn not_hoverable_not_hovering() {
            let mut app = App::new();

            app.add_plugins(TestPlugin(Vec2::new(2000., 0.)));

            app.add_systems(
                OnEnter(TestStates::Loaded),
                (
                    |images: Res<TestImageAssets>, mut commands: Commands| {
                        commands.spawn((
                            Sprite::from_image(images.card_back.clone_weak()),
                            ScaledSize::default(),
                            Transform::from_xyz(0., 0., 0.),
                        ));
                    },
                    is_hovered,
                    |entity_query: Query<(), With<Hovered>>, mut exit: EventWriter<AppExit>| {
                        assert!(entity_query.is_empty());
                        exit.send(AppExit::Success);
                    },
                )
                    .chain(),
            );

            app.run();
        }
    }
}
