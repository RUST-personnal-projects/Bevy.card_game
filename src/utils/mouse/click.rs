use bevy::{
    color::palettes::css,
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
};

use super::hover::Hovered;

#[derive(Component, Debug, Default)]
pub(crate) struct Clickable;

#[derive(Component, Debug)]
pub(crate) struct Clicked;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (is_clicked, is_released, gizmo));
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
                css::GREEN,
            );
        }
    }
}

/// Get Clickable components that are hovered and add Clicked if left mouse press event is registered
fn is_clicked(
    mut entity_query: Query<Entity, (With<Hovered>, With<Clickable>)>,
    mut mouse_event: EventReader<MouseButtonInput>,
    mut commands: Commands,
) {
    for ev in mouse_event.read() {
        for entity in entity_query.iter_mut() {
            if ev.button == MouseButton::Left && ev.state.is_pressed() {
                commands.entity(entity).insert(Clicked);
            }
        }
    }
}

/// Get Clicked components and remove it if left mouse release event is registered
fn is_released(
    mut entity_query: Query<Entity, With<Clicked>>,
    mut mouse_event: EventReader<MouseButtonInput>,
    mut commands: Commands,
) {
    for ev in mouse_event.read() {
        for entity in entity_query.iter_mut() {
            if ev.button == MouseButton::Left && ev.state == ButtonState::Released {
                commands.entity(entity).remove::<Clicked>();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod is_clicked {
        use super::*;

        #[test]
        // Clickable [X] Hovered [V]
        fn not_clickable_hovered() {
            // Setup app
            let mut app = App::new();

            // Add mouse click event listener
            app.add_event::<MouseButtonInput>();

            // Create window to be clicked
            let window_id = app.world_mut().spawn(Window::default()).id();

            // Send mouse click event
            app.world_mut().send_event(MouseButtonInput {
                button: MouseButton::Left,
                state: ButtonState::Pressed,
                window: window_id,
            });

            // Add Clickable entity that is not Clickable but hovered
            let entity_id = app.world_mut().spawn(Hovered).id();

            // Add our system
            app.add_systems(Update, is_clicked);

            // update the game once to run the system
            app.update();

            // retrieve entity after update
            let entity = app.world().get_entity(entity_id);

            assert!(entity.is_some());
            assert!(!entity.unwrap().contains::<Clicked>());
        }

        #[test]
        // Clicked [V] Hovered [V] x2
        fn clicked_hovered_more_than_one() {
            // Setup app
            let mut app = App::new();

            // Add mouse click event listener
            app.add_event::<MouseButtonInput>();

            // Create window to be clicked
            let window_id = app.world_mut().spawn(Window::default()).id();

            // Send mouse click event
            app.world_mut().send_event(MouseButtonInput {
                button: MouseButton::Left,
                state: ButtonState::Pressed,
                window: window_id,
            });

            // Add Clickable entity that is also Hovered
            let entity_id = app.world_mut().spawn((Clickable, Hovered)).id();

            // Add second Clickable entity that is also Hovered
            let second_entity_id = app.world_mut().spawn((Clickable, Hovered)).id();

            // Add our system
            app.add_systems(Update, is_clicked);

            // update the game once to run the system
            app.update();

            // retrieve entity after update
            let entity = app.world().get_entity(entity_id);
            let second_entity = app.world().get_entity(second_entity_id);

            assert!(entity.is_some());
            assert!(entity.unwrap().contains::<Clicked>());
            assert!(second_entity.unwrap().contains::<Clicked>());
        }

        #[test]
        // Clicked [V] Hovered [V]
        fn clicked_hovered() {
            // Setup app
            let mut app = App::new();

            // Add mouse click event listener
            app.add_event::<MouseButtonInput>();

            // Create window to be clicked
            let window_id = app.world_mut().spawn(Window::default()).id();

            // Send mouse click event
            app.world_mut().send_event(MouseButtonInput {
                button: MouseButton::Left,
                state: ButtonState::Pressed,
                window: window_id,
            });

            // Add Clickable entity that is also Hovered
            let entity_id = app.world_mut().spawn((Clickable, Hovered)).id();

            // Add our system
            app.add_systems(Update, is_clicked);

            // update the game once to run the system
            app.update();

            // retrieve entity after update
            let entity = app.world().get_entity(entity_id);

            assert!(entity.is_some());
            assert!(entity.unwrap().contains::<Clicked>());
        }

        #[test]
        // Clicked [X] Hovered [V]
        fn not_clicked_hovered() {
            // Setup app
            let mut app = App::new();

            // Add mouse click event listener
            app.add_event::<MouseButtonInput>();

            // Add Clickable entity that is also Hovered
            let entity_id = app.world_mut().spawn((Clickable, Hovered)).id();

            // Add our system
            app.add_systems(Update, is_clicked);

            // update the game once to run the system
            app.update();

            // retrieve entity after update
            let entity = app.world().get_entity(entity_id);

            assert!(entity.is_some());
            assert!(!entity.unwrap().contains::<Clicked>());
        }

        #[test]
        // Clicked [V] Hovered [X]
        fn clicked_not_hovered() {
            // Setup app
            let mut app = App::new();

            // Add mouse click event listener
            app.add_event::<MouseButtonInput>();

            // Create window to be clicked
            let window_id = app.world_mut().spawn(Window::default()).id();

            // Send mouse click event
            app.world_mut().send_event(MouseButtonInput {
                button: MouseButton::Left,
                state: ButtonState::Pressed,
                window: window_id,
            });

            // Add Clickable entity that is not Hovered
            let entity_id = app.world_mut().spawn(Clickable).id();

            // Add our system
            app.add_systems(Update, is_clicked);

            // update the game once to run the system
            app.update();

            // retrieve entity after update
            let entity = app.world().get_entity(entity_id);

            assert!(entity.is_some());
            assert!(!entity.unwrap().contains::<Clicked>());
        }

        #[test]
        // Clicked [X] Hovered [X]
        fn not_clicked_not_hovered() {
            // Setup app
            let mut app = App::new();

            // Add mouse click event listener
            app.add_event::<MouseButtonInput>();

            // Add Clickable entity that is not Hovered
            let entity_id = app.world_mut().spawn(Clickable).id();

            // Add our system
            app.add_systems(Update, is_clicked);

            // update the game once to run the system
            app.update();

            // retrieve entity after update
            let entity = app.world().get_entity(entity_id);

            assert!(entity.is_some());
            assert!(!entity.unwrap().contains::<Clicked>());
        }
    }

    mod is_released {
        use super::*;

        #[test]
        // Released [V] Clicked [V] x2
        fn released_clicked_more_than_one() {
            // Setup app
            let mut app = App::new();

            // Add mouse click event listener
            app.add_event::<MouseButtonInput>();

            // Create window to be clicked
            let window_id = app.world_mut().spawn(Window::default()).id();

            // Send mouse click event
            app.world_mut().send_event(MouseButtonInput {
                button: MouseButton::Left,
                state: ButtonState::Released,
                window: window_id,
            });

            // Add Clicked entity
            let entity_id = app.world_mut().spawn(Clicked).id();

            // Add second Clicked entity
            let second_entity_id = app.world_mut().spawn(Clicked).id();

            // Add our system
            app.add_systems(Update, is_released);

            // update the game once to run the system
            app.update();

            // retrieve entity after update
            let entity = app.world().get_entity(entity_id);

            // retrieve second entity after update
            let second_entity = app.world().get_entity(second_entity_id);

            assert!(entity.is_some());
            assert!(!entity.unwrap().contains::<Clicked>());
            assert!(!second_entity.unwrap().contains::<Clicked>());
        }

        #[test]
        // Released [V] Clicked [V]
        fn released_clicked() {
            // Setup app
            let mut app = App::new();

            // Add mouse click event listener
            app.add_event::<MouseButtonInput>();

            // Create window to be clicked
            let window_id = app.world_mut().spawn(Window::default()).id();

            // Send mouse click event
            app.world_mut().send_event(MouseButtonInput {
                button: MouseButton::Left,
                state: ButtonState::Released,
                window: window_id,
            });

            // Add Clicked entity
            let entity_id = app.world_mut().spawn(Clicked).id();

            // Add our system
            app.add_systems(Update, is_released);

            // update the game once to run the system
            app.update();

            // retrieve entity after update
            let entity = app.world().get_entity(entity_id);

            assert!(entity.is_some());
            assert!(!entity.unwrap().contains::<Clicked>());
        }

        #[test]
        // Released [X] Clicked [V]
        fn not_released_clicked() {
            // Setup app
            let mut app = App::new();

            // Add mouse click event listener
            app.add_event::<MouseButtonInput>();

            // Add Clicked entity
            let entity_id = app.world_mut().spawn(Clicked).id();

            // Add our system
            app.add_systems(Update, is_released);

            // update the game once to run the system
            app.update();

            // retrieve entity after update
            let entity = app.world().get_entity(entity_id);

            assert!(entity.is_some());
            assert!(entity.unwrap().contains::<Clicked>());
        }

        #[test]
        // Released [V] Clicked [X]
        fn released_not_clicked() {
            // Setup app
            let mut app = App::new();

            // Add mouse click event listener
            app.add_event::<MouseButtonInput>();

            // Create window to be clicked
            let window_id = app.world_mut().spawn(Window::default()).id();

            // Send mouse click event
            app.world_mut().send_event(MouseButtonInput {
                button: MouseButton::Left,
                state: ButtonState::Released,
                window: window_id,
            });

            // Add entity
            let entity_id = app.world_mut().spawn(Clickable).id();

            // Add our system
            app.add_systems(Update, is_released);

            // update the game once to run the system
            app.update();

            // retrieve entity after update
            let entity = app.world().get_entity(entity_id);

            assert!(entity.is_some());
            assert!(!entity.unwrap().contains::<Clicked>());
        }

        #[test]
        // Released [X] Clicked [X]
        fn not_released_not_clicked() {
            // Setup app
            let mut app = App::new();

            // Add mouse click event listener
            app.add_event::<MouseButtonInput>();

            // Add entity
            let entity_id = app.world_mut().spawn(Clickable).id();

            // Add our system
            app.add_systems(Update, is_released);

            // update the game once to run the system
            app.update();

            // retrieve entity after update
            let entity = app.world().get_entity(entity_id);

            assert!(entity.is_some());
            assert!(!entity.unwrap().contains::<Clicked>());
        }
    }
}
