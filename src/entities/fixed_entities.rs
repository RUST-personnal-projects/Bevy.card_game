use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

use crate::{screens::Screen, utils::image_scaling::ScaleOrderSet};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            init_fixed_scales,
            update_fixed_scales_on_window_resize,
            init_fixed_positions,
            update_fixed_positions_on_window_resize,
        )
            .in_set(ScaleOrderSet::UpdateScale)
            .run_if(in_state(Screen::Playing)),
    );
}

/// Add this [`Component`] to any [`Entity`] that needs to be scaled depending on window size
/// Express values from 0 to 1 as a percentage of the window, 0 meaning 0% of the window and 1 meaning 100% of the window
#[derive(Component)]
pub struct FixedScale {
    scale_ratio: Vec2,
}

impl FixedScale {
    pub const CARD: Self = Self::with_ratio(0.15, 0.15);

    const fn with_ratio(x_ratio: f32, y_ratio: f32) -> Self {
        Self {
            scale_ratio: Vec2::new(x_ratio, y_ratio),
        }
    }

    fn updated_scale(&self, width: f32, height: f32, image_size: Vec2) -> Vec2 {
        let scale_x = (width * self.scale_ratio.x) / image_size.x;
        let scale_y = (height * self.scale_ratio.y) / image_size.y;

        // Use the smaller scale factor to maintain aspect ratio
        Vec2::splat(scale_x.min(scale_y))
    }
}

fn init_fixed_scales(
    mut query: Query<(&mut Transform, &Handle<Image>, &FixedScale), Added<FixedScale>>,
    images: Res<Assets<Image>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for (mut transform, image_handle, fixed_scale) in query.iter_mut() {
        if let Some(image) = images.get(image_handle) {
            let window_size = window_query.single().resolution.size();
            let image_size = image.size_f32();

            transform.scale = fixed_scale
                .updated_scale(window_size.x, window_size.y, image_size)
                .extend(1.);
        } else {
            warn!(
                "Assets server returned None for {:?}, unable to scale entity.",
                image_handle
            );
        }
    }
}

/// Query all entities that have both [`Transform`] and [`Handle<Image>`] and update their scale to match the new window size
fn update_fixed_scales_on_window_resize(
    mut query: Query<(&mut Transform, &Handle<Image>, &FixedScale)>,
    images: Res<Assets<Image>>,
    mut events: EventReader<WindowResized>,
) {
    for WindowResized { height, width, .. } in events.read() {
        for (mut transform, image_handle, fixed_scale) in query.iter_mut() {
            if let Some(image) = images.get(image_handle) {
                let image_size = image.size_f32();

                transform.scale = fixed_scale
                    .updated_scale(*width, *height, image_size)
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

/// Add this [`Component`] to any [`Entity`] that needs to keep the same position on any window size
/// Express values from -1 to 1 as a ratio of the window area starting from the middle
/// Diagram:
///
///_______________1_______________
///|                              |
///|-1            0              1|
///|                              |
///_______________-1_______________
#[derive(Component)]
pub struct FixedPosition {
    position_ratio: Vec2,
}

impl FixedPosition {
    pub const DECK: Self = Self::with_ratio(0.15, 0.);
    pub const GRAVEYARD: Self = Self::with_ratio(-0.15, 0.);
    pub const HAND: Self = Self::with_ratio(0., -0.5);

    const fn with_ratio(x_ratio: f32, y_ratio: f32) -> Self {
        Self {
            position_ratio: Vec2::new(x_ratio, y_ratio),
        }
    }

    fn updated_translation(&self, width: f32, height: f32) -> Vec2 {
        // Dividing by two since (0., 0.) represents the middle of the window
        let translate_x = (width * self.position_ratio.x) / 2.;
        let translate_y = (height * self.position_ratio.y) / 2.;

        Vec2::new(translate_x, translate_y)
    }
}

fn init_fixed_positions(
    mut query: Query<(&mut Transform, &FixedPosition), Added<FixedPosition>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for (mut transform, fixed_position) in query.iter_mut() {
        let window_size = window_query.single().resolution.size();
        transform.translation = fixed_position
            .updated_translation(window_size.x, window_size.y)
            .extend(transform.translation.z);
    }
}

/// Query all entities that have [`Transform`] and update their positions to match the new window size
fn update_fixed_positions_on_window_resize(
    mut query: Query<(&mut Transform, &FixedPosition)>,
    mut events: EventReader<WindowResized>,
) {
    for WindowResized { height, width, .. } in events.read() {
        for (mut transform, fixed_position) in query.iter_mut() {
            transform.translation = fixed_position
                .updated_translation(*width, *height)
                .extend(1.);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_fixed_scale {
        use super::*;

        #[test]
        fn positive_scale() {
            let entity = FixedScale::with_ratio(1., 1.);
            let updated_scale = entity.updated_scale(1000., 1000., Vec2::ONE);

            assert_eq!(Vec2::new(1000., 1000.), updated_scale);
        }

        #[test]
        fn null_scale() {
            let entity = FixedScale::with_ratio(0., 0.);
            let updated_scale = entity.updated_scale(1000., 1000., Vec2::ONE);

            assert_eq!(Vec2::new(0., 0.), updated_scale);
        }

        #[test]
        fn negative_scale() {
            let entity = FixedScale::with_ratio(-1., -1.);
            let updated_scale = entity.updated_scale(1000., 1000., Vec2::ONE);

            assert_eq!(Vec2::new(-1000., -1000.), updated_scale);
        }

        #[test]
        fn negative_screen_size() {
            let entity = FixedScale::with_ratio(1., 1.);
            let updated_scale = entity.updated_scale(-1000., -1000., Vec2::ONE);

            assert_eq!(Vec2::new(-1000., -1000.), updated_scale);
        }

        #[test]
        fn negative_sprite_size() {
            let entity = FixedScale::with_ratio(1., 1.);
            let updated_scale = entity.updated_scale(1000., 1000., Vec2::NEG_ONE);

            assert_eq!(Vec2::new(-1000., -1000.), updated_scale);
        }

        #[test]
        fn positive_smallest_scale() {
            let entity = FixedScale::with_ratio(0.5, 1.);
            let updated_scale = entity.updated_scale(1000., 1000., Vec2::ONE);

            assert_eq!(Vec2::new(500., 500.), updated_scale);
        }

        #[test]
        fn negative_smallest_scale() {
            let entity = FixedScale::with_ratio(-0.5, -1.);
            let updated_scale = entity.updated_scale(1000., 1000., Vec2::ONE);

            assert_eq!(Vec2::new(-1000., -1000.), updated_scale);
        }
    }

    mod test_fixed_position {
        use super::*;

        #[test]
        fn top_right_position() {
            let entity = FixedPosition::with_ratio(1., 1.);
            let updated_translation = entity.updated_translation(1000., 1000.);

            assert_eq!(Vec2::new(500., 500.), updated_translation);
        }

        #[test]
        fn middle_top_position() {
            let entity = FixedPosition::with_ratio(0., 1.);
            let updated_translation = entity.updated_translation(1000., 1000.);

            assert_eq!(Vec2::new(0., 500.), updated_translation);
        }

        #[test]
        fn top_left_position() {
            let entity = FixedPosition::with_ratio(-1., 1.);
            let updated_translation = entity.updated_translation(1000., 1000.);

            assert_eq!(Vec2::new(-500., 500.), updated_translation);
        }

        #[test]
        fn middle_right_position() {
            let entity = FixedPosition::with_ratio(1., 0.);
            let updated_translation = entity.updated_translation(1000., 1000.);

            assert_eq!(Vec2::new(500., 0.), updated_translation);
        }

        #[test]
        fn middle_position() {
            let entity = FixedPosition::with_ratio(0., 0.);
            let updated_translation = entity.updated_translation(1000., 1000.);

            assert_eq!(Vec2::new(0., 0.), updated_translation);
        }

        #[test]
        fn middle_left_position() {
            let entity = FixedPosition::with_ratio(-1., 0.);
            let updated_translation = entity.updated_translation(1000., 1000.);

            assert_eq!(Vec2::new(-500., 0.), updated_translation);
        }

        #[test]
        fn bottom_right_position() {
            let entity = FixedPosition::with_ratio(1., -1.);
            let updated_translation = entity.updated_translation(1000., 1000.);

            assert_eq!(Vec2::new(500., -500.), updated_translation);
        }

        #[test]
        fn middle_bottom_position() {
            let entity = FixedPosition::with_ratio(0., -1.);
            let updated_translation = entity.updated_translation(1000., 1000.);

            assert_eq!(Vec2::new(0., -500.), updated_translation);
        }

        #[test]
        fn bottom_left_position() {
            let entity = FixedPosition::with_ratio(-1., -1.);
            let updated_translation = entity.updated_translation(1000., 1000.);

            assert_eq!(Vec2::new(-500., -500.), updated_translation);
        }

        #[test]
        fn outside_area_position() {
            let entity = FixedPosition::with_ratio(-1.5, -1.);
            let updated_translation = entity.updated_translation(1000., 1000.);

            assert_eq!(Vec2::new(-750., -500.), updated_translation);
        }
    }
}
