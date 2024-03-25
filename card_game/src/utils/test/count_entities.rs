use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct EntityCount(pub usize);

/// Counts how many entities with T Component currently exist in the world
pub fn count_entities<T: Component>(
    query: Query<(), With<T>>,
    mut entity_count: ResMut<EntityCount>,
) {
    entity_count.0 = query.iter().len();
}
