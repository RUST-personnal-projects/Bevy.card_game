use bevy::{prelude::*, window::WindowResized};

use crate::screens::Screen;

use super::GameEntity;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (update_entities_scale, update_entities_translation)
            .chain()
            .run_if(in_state(Screen::Playing)),
    );
}

/// Impl this trait to any types that need to be scaled on window resize
/// Express values from 0 to 1 as a percentage of the window, 0 meaning 0% of the window and 1 meaning 100% of the window
pub(crate) trait Scalable: Component {
    fn scale(&self) -> Vec2;

    fn update_scale(&self, width: f32, height: f32, image_size: Vec2) -> Vec2 {
        let scale_ratio = self.scale();

        let scale_x = (width * scale_ratio.x) / image_size.x;
        let scale_y = (height * scale_ratio.y) / image_size.y;

        // Use the smaller scale factor to maintain aspect ratio
        Vec2::splat(scale_x.min(scale_y))
    }
}

/// Query all entities that have both [`Transform`] and [`Handle<Image>`] and update their scale to match the window size
fn update_entities_scale(
    mut query: Query<(&mut Transform, &Handle<Image>, &GameEntity)>,
    images: Res<Assets<Image>>,
    mut events: EventReader<WindowResized>,
) {
    for WindowResized { height, width, .. } in events.read() {
        for (mut transform, image_handle, entity_scale_ratio) in query.iter_mut() {
            if let Some(image) = images.get(image_handle) {
                let image_size = image.size_f32();

                transform.scale = entity_scale_ratio
                    .update_scale(*width, *height, image_size)
                    .extend(1.);
            } else {
                warn!(
                    "Assets server returned None for {:?}, unable to scale entity.",
                    image_handle
                );
            }
        }
    }
}

/// Impl this trait to any types that need to be translated on window resize
/// Express values from -1 to 1 as a percentage of the window area starting from the middle
/// Diagram:
///
///_______________+100%_______________
///|                                 |
///|-100%          0%           +100%|
///|                                 |
///_______________-100%_______________
pub(crate) trait Translatable: Component {
    fn get_translation(&self) -> Vec2;

    fn update_translation(&self, width: f32, height: f32) -> Vec2 {
        let translate_ratio = self.get_translation();

        // Dividing by two since (0., 0.) represents the middle of the window
        let translate_x = (width * translate_ratio.x) / 2.;
        let translate_y = (height * translate_ratio.y) / 2.;

        Vec2::new(translate_x, translate_y)
    }
}

fn update_entities_translation(
    mut query: Query<(&mut Transform, &GameEntity)>,
    mut events: EventReader<WindowResized>,
) {
    for WindowResized { height, width, .. } in events.read() {
        for (mut transform, game_entity) in query.iter_mut() {
            transform.translation = game_entity.update_translation(*width, *height).extend(1.);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_update_entities_scale {
        use super::*;

        #[derive(Component)]
        enum TestEntity {
            Full,
            Half,
        }

        impl Scalable for TestEntity {
            fn scale(&self) -> Vec2 {
                match self {
                    // This would mean that the entity should take 100% of the window area
                    TestEntity::Full => Vec2::new(1., 1.),
                    // This would mean that the entity should take 50% of the window area
                    TestEntity::Half => Vec2::new(0.5, 0.5),
                }
            }
        }

        #[test]
        fn scale_on_window_resized_full() {
            let entity = TestEntity::Full;

            let updated_scale = entity.update_scale(1000., 1000., Vec2::new(100., 100.));

            // An entity taking 100% if the screen and image_size being 100x100 pixels would need to be scaled x10 to scale
            assert_eq!((10., 10.), (updated_scale.x, updated_scale.y));
        }

        #[test]
        fn scale_on_window_resized_half() {
            let entity = TestEntity::Half;

            let updated_scale = entity.update_scale(1000., 1000., Vec2::new(100., 100.));

            // An entity taking 50% if the screen and image_size being 100x100 pixels would need to be scaled x5 to scale
            assert_eq!((5., 5.), (updated_scale.x, updated_scale.y));
        }
    }

    mod test_update_entities_translate {

        use super::*;

        #[derive(Component)]
        struct TestEntity;

        impl Translatable for TestEntity {
            fn get_translation(&self) -> Vec2 {
                Vec2::new(1., 1.)
            }
        }

        #[test]
        fn translate_on_window_resized() {
            let entity = TestEntity;
            let updated_translation = entity.update_translation(1000., 1000.);

            // Assert that on a 1000 by 1000 window
            assert_eq!(Vec2::new(500., 500.), updated_translation);
        }
    }
}
