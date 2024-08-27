use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_scaled_size);
}

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub(crate) struct ScaledSize(pub Vec2);

impl ScaledSize {
    pub(crate) fn width(&self) -> f32 {
        self.x
    }

    pub(crate) fn height(&self) -> f32 {
        self.y
    }
}

fn update_scaled_size(
    mut hoverables_query: Query<(&Handle<Image>, &Transform, &mut ScaledSize)>,
    images: Res<Assets<Image>>,
) {
    for (image, transform, mut scaled_size) in hoverables_query.iter_mut() {
        if let Some(image) = images.get(image) {
            **scaled_size = image.size_f32() * transform.scale.truncate();
        }
    }
}
