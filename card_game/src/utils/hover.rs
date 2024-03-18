use bevy::prelude::*;

use super::mouse::MouseCoordinates;

#[derive(Component, Debug)]
pub struct Hoverable;

#[derive(Component, Debug)]
pub struct Hovered;

pub struct HoverPlugin;

impl Plugin for HoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (is_hovered, hovered_gizmo));
    }
}

fn hovered_gizmo(
    mut gizmos: Gizmos,
    hoverables_query: Query<(&Handle<Image>, &Transform), With<Hovered>>,
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
    hoverables_query: Query<(Entity, &Handle<Image>, &Transform), With<Hoverable>>,
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
