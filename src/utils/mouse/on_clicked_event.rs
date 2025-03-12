use bevy::prelude::*;

use super::{click::Clickable, hover::Hovered};

#[derive(Event, PartialEq, Deref, DerefMut)]
pub struct OnEntityClickedEvent(pub Entity);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, on_click_event)
        .add_event::<OnEntityClickedEvent>();
}

fn on_click_event(
    buttons: Res<ButtonInput<MouseButton>>,
    mut on_click_ev_writer: EventWriter<OnEntityClickedEvent>,
    entity_query: Query<Entity, (With<Hovered>, With<Clickable>)>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for entity in entity_query.iter() {
            on_click_ev_writer.send(OnEntityClickedEvent(entity));
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::input::mouse::MouseButtonInput;

    use super::*;

    #[test]
    fn on_clicked() {
        // Setup app
        let mut app = App::new();
        app.init_resource::<ButtonInput<MouseButton>>()
            .add_event::<MouseButtonInput>()
            .add_event::<OnEntityClickedEvent>();

        // Send mouse click event
        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Left);

        // Add Clickable entity
        let entity = app.world_mut().spawn((Clickable, Hovered)).id();

        // Add our system
        app.add_systems(Update, on_click_event);

        // update the game once to run the system
        app.update();

        // retrieve entity after update
        let event_reader = app.world().resource::<Events<OnEntityClickedEvent>>();

        assert_eq!(event_reader.len(), 1);
        for &OnEntityClickedEvent(event_entity) in event_reader.iter_current_update_events() {
            assert!(event_entity == entity);
        }
    }

    #[test]
    fn on_clicked_multiple() {
        // Setup app
        let mut app = App::new();
        app.init_resource::<ButtonInput<MouseButton>>()
            .add_event::<MouseButtonInput>()
            .add_event::<OnEntityClickedEvent>();

        // Send mouse click events
        let mut mouse_input = app.world_mut().resource_mut::<ButtonInput<MouseButton>>();
        mouse_input.press(MouseButton::Left);
        mouse_input.press(MouseButton::Left);
        mouse_input.press(MouseButton::Left);

        // Add Clickable entity
        let entity = app.world_mut().spawn((Clickable, Hovered)).id();

        // Add our system
        app.add_systems(Update, on_click_event);

        // update the game once to run the system
        app.update();

        // retrieve entity after update
        let event_reader = app.world().resource::<Events<OnEntityClickedEvent>>();

        assert_eq!(event_reader.len(), 1);
        for &OnEntityClickedEvent(event_entity) in event_reader.iter_current_update_events() {
            assert!(event_entity == entity);
        }
    }

    #[test]
    fn on_clicked_none() {
        // Setup app
        let mut app = App::new();
        app.init_resource::<ButtonInput<MouseButton>>()
            .add_event::<MouseButtonInput>()
            .add_event::<OnEntityClickedEvent>();

        // Add Clickable entity
        app.world_mut().spawn((Clickable, Hovered));

        // Add our system
        app.add_systems(Update, on_click_event);

        // update the game once to run the system
        app.update();

        // retrieve entity after update
        let event_reader = app.world().resource::<Events<OnEntityClickedEvent>>();

        assert!(event_reader.is_empty());
    }
}
