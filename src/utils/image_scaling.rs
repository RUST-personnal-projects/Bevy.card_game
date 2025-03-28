use bevy::prelude::*;

// This system allows systems that depend on scale being set up before running
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScaleOrderSet {
    UpdateScale,
    CalculateScaledSize,
    UseScaledSize,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        update_scaled_size.in_set(ScaleOrderSet::CalculateScaledSize),
    )
    .register_type::<ScaledSize>()
    .configure_sets(
        Update,
        (
            ScaleOrderSet::UpdateScale,
            ScaleOrderSet::CalculateScaledSize,
            ScaleOrderSet::UseScaledSize,
        )
            .chain(),
    );
}

/// This component allows any entity with [`Transform`] and [`Handle<Image>`] components to automatically have access to a scaled [`Vec2`] of it's size
/// Important: this component is not here to ensure scale is respected, it just helps calculating image size depending on transform scale
#[derive(Component, Debug, Default, Deref, DerefMut, Reflect)]
pub struct ScaledSize(pub Vec2);

impl ScaledSize {
    pub fn width(&self) -> f32 {
        self.x
    }

    pub fn height(&self) -> f32 {
        self.y
    }

    pub fn set_scaled_size(
        &mut self,
        transform: &Transform,
        image_handle: &Handle<Image>,
        images: &Res<Assets<Image>>,
    ) {
        if let Some(image) = images.get(image_handle) {
            self.0 = image.size_f32() * transform.scale.truncate();
        }
    }
}

fn update_scaled_size(
    mut hoverables_query: Query<(&Sprite, &Transform, &mut ScaledSize), Changed<Transform>>,
    images: Res<Assets<Image>>,
) {
    for (sprite, transform, mut scaled_size) in hoverables_query.iter_mut() {
        scaled_size.set_scaled_size(transform, &sprite.image, &images)
    }
}
