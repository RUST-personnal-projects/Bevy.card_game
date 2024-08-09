#[cfg(test)]
use std::any::type_name;
use std::marker::PhantomData;

#[cfg(test)]
use bevy::asset::LoadState;
use bevy::prelude::*;

#[derive(Component)]
pub struct Loaded<T: Asset> {
    _type: PhantomData<T>,
}

impl<T: Asset> Loaded<T> {
    #[cfg(test)]
    fn new() -> Self {
        Self { _type: PhantomData }
    }
}

#[cfg(test)]
pub fn is_asset_loaded<T: Asset>(
    assets_query: Query<(Entity, &Handle<T>), Without<Loaded<T>>>,
    assets: Res<AssetServer>,
    mut commands: Commands,
) {
    for (entity, asset) in assets_query.iter() {
        let load_state = assets.load_state(asset);
        match load_state {
            LoadState::Loaded => {
                info!(
                    "Finished loading asset {:?} of type {:?} for entity {:?}.",
                    asset,
                    type_name::<T>(),
                    entity
                );
                commands.entity(entity).insert(Loaded::<T>::new());
            }
            LoadState::Failed(err) => warn!(
                "Couldn't load asset {:?} of type {:?} for entity {:?} with error: {:?}",
                asset,
                type_name::<T>(),
                entity,
                err,
            ),
            _ => {}
        }
    }
}
