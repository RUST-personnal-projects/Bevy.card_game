use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
};

use super::hover::Hovered;

#[derive(Component, Debug, Default)]
pub struct Clickable;

#[derive(Component, Debug)]
pub struct Clicked;

pub struct ClickPlugin;

impl Plugin for ClickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (is_clicked, is_released, gizmo));
    }
}

fn gizmo(
    mut gizmos: Gizmos,
    hoverables_query: Query<(&Handle<Image>, &Transform), With<Clicked>>,
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
                Color::BLUE,
            );
        }
    }
}

fn is_released(
    mut entity_query: Query<Entity, With<Clicked>>,
    mut mouse_event: EventReader<MouseButtonInput>,
    mut commands: Commands,
) {
    if let Ok(entity) = entity_query.get_single_mut() {
        for ev in mouse_event.read() {
            if ev.button == MouseButton::Left && ev.state == ButtonState::Released {
                commands.entity(entity).remove::<Clicked>();
            }
        }
    }
}

fn is_clicked(
    mut entity_query: Query<Entity, With<Hovered>>,
    mut mouse_event: EventReader<MouseButtonInput>,
    mut commands: Commands,
) {
    if let Ok(entity) = entity_query.get_single_mut() {
        for ev in mouse_event.read() {
            if ev.button == MouseButton::Left && ev.state.is_pressed() {
                commands.entity(entity).insert(Clicked);
            }
        }
    }
}
