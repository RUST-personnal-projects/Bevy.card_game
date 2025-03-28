//! Helper traits for creating common widgets.

use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use crate::utils;

use super::{interaction::InteractionPalette, palette::*};

/// An extension trait for spawning UI widgets.
pub trait Widgets {
    /// Spawn a simple button with text.
    fn button(
        &mut self,
        text: impl Into<String>,
        font: Option<Handle<Font>>,
        button_image: Option<Handle<Image>>,
    ) -> EntityCommands;

    /// Spawn a simple text label.
    fn label(&mut self, text: impl Into<String>) -> EntityCommands;
}

impl<T: Spawn> Widgets for T {
    fn button(
        &mut self,
        text: impl Into<String>,
        font: Option<Handle<Font>>,
        button_image: Option<Handle<Image>>,
    ) -> EntityCommands {
        use Spawn;
        let button_node = Node {
            width: Px(200.0),
            height: Px(65.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        let button = if let Some(button_image) = button_image {
            (
                Button,
                button_node,
                ImageNode::new(button_image),
                BackgroundColor::default(),
            )
        } else {
            (
                Button,
                button_node,
                ImageNode::default(),
                BackgroundColor(NODE_BACKGROUND),
            )
        };

        self.spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        });

        let mut entity = self.spawn((
            Name::new("Button"),
            button,
            InteractionPalette {
                none: NORMAL_BUTTON,
                hovered: HOVERED_BUTTON,
                pressed: PRESSED_BUTTON,
            },
        ));

        entity.with_children(|children| {
            let style = (
                TextFont {
                    font: font.unwrap_or_default(),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(BUTTON_TEXT),
            );

            Spawn::spawn(children, (Name::new("Button Text"), Text::new(text), style));
        });
        entity
    }

    fn label(&mut self, text: impl Into<String>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Label"),
            Node {
                width: Px(250.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ));
        entity.with_children(|children| {
            Spawn::spawn(
                children,
                (
                    Name::new("Label Text"),
                    Text::new(text),
                    TextFont::from_font_size(24.),
                    TextColor(LABEL_TEXT),
                ),
            );
        });
        entity
    }
}

/// An extension trait for spawning UI containers.
pub trait Containers {
    /// Spawns a root node that covers the full screen
    /// and centers its content horizontally and vertically.
    fn ui_root(
        &mut self,
        transform: Option<Transform>,
        flex_direction: Option<FlexDirection>,
    ) -> EntityCommands;
}

impl Containers for Commands<'_, '_> {
    fn ui_root(
        &mut self,
        transform: Option<Transform>,
        flex_direction: Option<FlexDirection>,
    ) -> EntityCommands {
        self.spawn((
            Name::new("UI Root"),
            Node {
                width: Percent(100.0),
                height: Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: flex_direction.unwrap_or(FlexDirection::Column),
                row_gap: Px(10.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            transform.unwrap_or_default(),
        ))
    }
}

/// An internal trait for types that can spawn entities.
/// This is here so that [`Widgets`] can be implemented on all types that
/// are able to spawn entities.
/// Ideally, this trait should be [part of Bevy itself](https://github.com/bevyengine/bevy/issues/14231).
trait Spawn {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl Spawn for ChildBuilder<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        bevy::prelude::ChildBuild::spawn(self, bundle)
    }
}
