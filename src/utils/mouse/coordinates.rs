use bevy::{color::palettes::css, input::mouse::MouseMotion, prelude::*};

#[derive(Resource, Default)]
pub(crate) struct MouseCoordinates(pub Vec2);

#[derive(Resource, Default)]
pub(crate) struct UIMouseCoordinates(pub Vec2);

#[derive(Component)]
struct TextCoordinatesMarker;

#[derive(Component)]
struct UITextCoordinatesMarker;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, (update_ui_coordinates, update_ui_coordinates_text))
        .add_systems(
            Update,
            (update_coordinates, update_coordinates_text).after(update_ui_coordinates),
        )
        .init_resource::<MouseCoordinates>()
        .init_resource::<UIMouseCoordinates>();
}

#[cfg(debug_assertions)]
fn setup(mut commands: Commands) {
    // UI node

    commands
        .spawn((NodeBundle {
            background_color: BackgroundColor(css::DARK_GRAY.into()),
            border_color: BorderColor(Color::BLACK),
            style: Style {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },))
        .with_children(|builder| {
            builder.spawn((
                // Text
                TextBundle::default(),
                TextCoordinatesMarker,
            ));
            builder.spawn((
                // Text
                TextBundle::default(),
                UITextCoordinatesMarker,
            ));
        });
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

#[cfg(debug_assertions)]
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

fn update_coordinates_text(
    mouse_coordinates: Res<MouseCoordinates>,
    mut text_query: Query<&mut Text, With<TextCoordinatesMarker>>,
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

#[cfg(debug_assertions)]
fn update_ui_coordinates_text(
    mouse_coordinates: Res<UIMouseCoordinates>,
    mut text_query: Query<&mut Text, With<UITextCoordinatesMarker>>,
) {
    let mut text = text_query.single_mut();

    *text = Text::from_section(
        format!(
            "UI mouse: \nx: {}\ny: {}",
            mouse_coordinates.0.x, mouse_coordinates.0.y
        ),
        TextStyle::default(),
    );
}
