use bevy::{input::mouse::MouseMotion, prelude::*};

#[cfg(feature = "dev")]
use crate::utils::dev_tools::{DebugViewOrderSet, DevState};

#[derive(Resource, Default)]
pub(crate) struct MouseCoordinates(pub Vec2);

#[derive(Resource, Default)]
pub(crate) struct UIMouseCoordinates(pub Vec2);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (update_ui_coordinates, update_coordinates))
        .init_resource::<MouseCoordinates>()
        .init_resource::<UIMouseCoordinates>();
    #[cfg(feature = "dev")]
    app.add_systems(
        Update,
        (
            debug::update_debug_mouse_coordinates.run_if(in_state(DevState::On)),
            debug::update_debug_ui_mouse_coordinates.run_if(in_state(DevState::On)),
        ),
    )
    .add_systems(
        Startup,
        debug::setup_mouse_debug_view.in_set(DebugViewOrderSet::Coordinates),
    );
}

fn update_coordinates(
    mut mouse_coordinates: ResMut<MouseCoordinates>,
    ui_mouse_coordinates: Res<UIMouseCoordinates>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut mouse_event: EventReader<MouseMotion>,
) {
    for _ in mouse_event.read() {
        let (camera, camera_transform) = camera_query.single();

        if let Some(coordinates) = camera
            .viewport_to_world_2d(camera_transform, ui_mouse_coordinates.0)
            .map(|coordinates| coordinates.trunc())
        {
            mouse_coordinates.0 = coordinates;
        }
    }
}

fn update_ui_coordinates(
    mut ui_mouse_coordinates: ResMut<UIMouseCoordinates>,
    window_query: Query<&Window>,
    mut mouse_event: EventReader<MouseMotion>,
) {
    for _ in mouse_event.read() {
        let window = window_query.single();

        if let Some(coordinates) = window.cursor_position() {
            ui_mouse_coordinates.0 = coordinates;
        }
    }
}

#[cfg(feature = "dev")]
mod debug {
    use super::*;

    use crate::utils::dev_tools::DebugNodeMarker;

    #[derive(Component)]
    pub(super) struct MouseCoordinatesMarker;

    #[derive(Component)]
    pub(super) struct UIMouseCoordinatesMarker;

    pub(super) fn setup_mouse_debug_view(
        mut commands: Commands,
        debug_node_query: Query<Entity, With<DebugNodeMarker>>,
    ) {
        let node = debug_node_query.single();

        let mouse_coordinates = commands
            .spawn((TextBundle::default(), MouseCoordinatesMarker))
            .id();

        let ui_mouse_coordinates = commands
            .spawn((TextBundle::default(), UIMouseCoordinatesMarker))
            .id();

        commands
            .entity(node)
            .push_children(&[mouse_coordinates, ui_mouse_coordinates]);
    }

    pub(super) fn update_debug_mouse_coordinates(
        mouse_coordinates: Res<MouseCoordinates>,
        mut text_query: Query<&mut Text, With<MouseCoordinatesMarker>>,
    ) {
        let mut mouse_coordinates_text = text_query.single_mut();

        *mouse_coordinates_text = Text::from_section(
            format!(
                "Mouse: \nx: {}\ny: {}",
                mouse_coordinates.0.x, mouse_coordinates.0.y
            ),
            TextStyle::default(),
        );
    }

    pub(super) fn update_debug_ui_mouse_coordinates(
        ui_mouse_coordinates: Res<UIMouseCoordinates>,
        mut text_query: Query<&mut Text, With<UIMouseCoordinatesMarker>>,
    ) {
        let mut ui_mouse_coordinates_text = text_query.single_mut();

        *ui_mouse_coordinates_text = Text::from_section(
            format!(
                "UI mouse: \nx: {}\ny: {}",
                ui_mouse_coordinates.0.x, ui_mouse_coordinates.0.y
            ),
            TextStyle::default(),
        );
    }
}
