//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{color, dev_tools::states::log_transitions, prelude::*};

use crate::game::screen::Screen;

#[derive(SubStates, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[source(Screen = Screen::Playing)]
pub(crate) enum DevState {
    #[default]
    Off,
    On,
}

#[derive(Component)]
pub(crate) struct DebugNodeMarker;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum DebugViewOrderSet {
    DevState,
    Coordinates,
}

pub(super) fn plugin(app: &mut App) {
    // Print state transitions in dev builds
    app.add_sub_state::<DevState>()
        .add_systems(
            Update,
            (
                switch_to_dev_mode.run_if(in_state(Screen::Playing)),
                toggle_debug.run_if(state_changed::<DevState>),
                log_transitions::<Screen>,
            ),
        )
        .configure_sets(
            Startup,
            (DebugViewOrderSet::DevState, DebugViewOrderSet::Coordinates).chain(),
        )
        .add_systems(Startup, setup_debug.in_set(DebugViewOrderSet::DevState));
}

fn switch_to_dev_mode(
    mut next_dev_state: ResMut<NextState<DevState>>,
    current_dev_state: Res<State<DevState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyF) {
        match current_dev_state.get() {
            DevState::Off => next_dev_state.set(DevState::On),
            DevState::On => next_dev_state.set(DevState::Off),
        }
    }
}

fn setup_debug(mut commands: Commands) {
    info!("Creating debug view");
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
    let mut visivility = debug_node_visibility_query.single_mut();

    *visivility = match *visivility {
        Visibility::Visible | Visibility::Inherited => Visibility::Hidden,
        Visibility::Hidden => Visibility::Visible,
    };
}
