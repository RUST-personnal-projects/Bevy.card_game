use bevy::prelude::*;

#[derive(Resource, Default)]
struct MouseCoordinates(Vec2);

#[derive(Component)]
struct MouseCoordinatesMarker;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    #[cfg(not(debug_assertions))]
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_coordinates)
            .init_resource::<MouseCoordinates>();
    }

    #[cfg(debug_assertions)]
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (update_coordinates, update_coordinates_text))
            .init_resource::<MouseCoordinates>();
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        TextBundle {
            transform: Transform::from_xyz(-500., 400., 0.),
            text: Text::from_section("mouse: ", TextStyle::default()),
            style: Style {
                top: Val::Px(5.),
                left: Val::Px(5.),
                ..default()
            },
            ..default()
        },
        MouseCoordinatesMarker,
    ));
}

fn update_coordinates(
    mut mouse_coordinates: ResMut<MouseCoordinates>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window>,
) {
    let (camera, camera_transform) = camera_query.single();

    let window = window_query.single();

    if let Some(coordinates) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        .map(|coordinates| coordinates.trunc())
    {
        mouse_coordinates.0 = coordinates;
    }
}

fn update_coordinates_text(
    mouse_coordinates: Res<MouseCoordinates>,
    mut text_query: Query<&mut Text, With<MouseCoordinatesMarker>>,
) {
    let mut text = text_query.single_mut();

    *text = Text::from_section(
        format!(
            "mouse: \nx: {}\ny: {}",
            mouse_coordinates.0.x, mouse_coordinates.0.y
        ),
        TextStyle::default(),
    );
}
