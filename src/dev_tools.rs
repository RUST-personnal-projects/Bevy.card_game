//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{color, dev_tools::states::log_transitions, prelude::*, window::PrimaryWindow};

use crate::screens::Screen;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
// #[source(Screen = Screen::Playing)]
pub enum DevState {
    #[default]
    Off,
    On,
}

#[derive(Component)]
pub struct DebugNodeMarker;

// This system set serves as a way to choose in which order dev info are printed
// When adding a new member to the set, don't forget to also add it to the configure_sets down bellow
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DebugViewOrderSet {
    DevState,
    Coordinates,
    Window,
    Player,
    Turn,
    Debug,
}

pub(super) fn plugin(app: &mut App) {
    // Print state transitions in dev builds, add a UI node showing various debug info
    app.init_state::<DevState>()
        .add_systems(
            Update,
            (
                switch_to_dev_mode.run_if(in_state(Screen::Playing)),
                update_debug_ui_mouse_coordinates.run_if(in_state(Screen::Playing)),
                toggle_debug.run_if(state_changed::<DevState>),
                log_transitions::<Screen>,
                log_transitions::<DevState>,
            ),
        )
        .configure_sets(
            Startup,
            (
                DebugViewOrderSet::DevState,
                DebugViewOrderSet::Coordinates,
                DebugViewOrderSet::Window,
                DebugViewOrderSet::Player,
                DebugViewOrderSet::Turn,
                DebugViewOrderSet::Debug,
            )
                .chain(),
        )
        .add_systems(
            Startup,
            (
                setup_debug.in_set(DebugViewOrderSet::DevState),
                setup_window_debug_view.in_set(DebugViewOrderSet::Window),
            ),
        );
}

fn switch_to_dev_mode(
    mut next_dev_state: ResMut<NextState<DevState>>,
    current_dev_state: Res<State<DevState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::F3) {
        match current_dev_state.get() {
            DevState::Off => next_dev_state.set(DevState::On),
            DevState::On => next_dev_state.set(DevState::Off),
        }
    }
}

fn setup_debug(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                background_color: BackgroundColor(color::palettes::css::DARK_GRAY.into()),
                border_color: BorderColor(Color::BLACK),
                style: Style {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            DebugNodeMarker,
        ))
        .with_children(|builder| {
            builder.spawn((TextBundle::from("DevState On"),));
        });
}

fn toggle_debug(mut debug_node_visibility_query: Query<&mut Visibility, With<DebugNodeMarker>>) {
    let mut visibility = debug_node_visibility_query.single_mut();

    *visibility = match *visibility {
        Visibility::Visible | Visibility::Inherited => Visibility::Hidden,
        Visibility::Hidden => Visibility::Visible,
    };
}

#[derive(Component)]
pub(super) struct UIWindowMarker;

fn setup_window_debug_view(
    mut commands: Commands,
    debug_node_query: Query<Entity, With<DebugNodeMarker>>,
) {
    let node = debug_node_query.single();

    let window = commands.spawn((TextBundle::default(), UIWindowMarker)).id();

    commands.entity(node).push_children(&[window]);
}

fn update_debug_ui_mouse_coordinates(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut ui_window_query: Query<&mut Text, With<UIWindowMarker>>,
) {
    let window_size = window_query.single().size();
    let mut ui_window_text = ui_window_query.single_mut();

    *ui_window_text = Text::from_section(
        format!("Window size: \nx: {}\ny: {}", window_size.x, window_size.y),
        TextStyle::default(),
    );
}
