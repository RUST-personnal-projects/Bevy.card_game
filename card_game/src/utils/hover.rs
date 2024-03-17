use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Hoverable;

#[derive(Component, Debug)]
pub struct Hovered;

pub struct HoverPlugin;

impl Plugin for HoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, is_hovered);
    }
}

fn is_hovered(
    hoverables_query: Query<(Entity, &Handle<Image>, &Transform), With<Hoverable>>,
    window_query: Query<&Window>,
    assets: Res<Assets<Image>>,
) {
    let window = window_query.single();
    for (entity, image, transform) in hoverables_query.iter() {
        if let Some(world_position) = window.cursor_position() {
            let mouse_x = world_position.x - window.resolution.physical_width() as f32 / 2.;
            let mouse_y = world_position.y - window.resolution.physical_height() as f32 / 2.;

            let image = assets.get(image).unwrap();
            let half_width = image.width() as f32 / 2.;
            let half_height = image.height() as f32 / 2.;

            let min_x = transform.translation.x - half_width;
            let max_x = transform.translation.x + half_width;
            let min_y = transform.translation.y - half_height;
            let max_y = transform.translation.y + half_height;
            bevy::log::info!("min x: {} | max x: {}", min_x, max_x);
            bevy::log::info!("min y: {} | max y: {}", min_y, max_y);

            if mouse_x >= min_x && mouse_x <= max_x && mouse_y >= min_y && mouse_y <= max_y {
                bevy::log::info!("Hovering: {:?}", entity);
            }
        }
    }
}
