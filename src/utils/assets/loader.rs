use std::{any::type_name, marker::PhantomData};

use bevy::{asset::LoadState, prelude::*};

#[derive(Component)]
pub struct Loaded<T: Asset> {
    _type: PhantomData<T>,
}

impl<T: Asset> Loaded<T> {
    fn new() -> Self {
        Self { _type: PhantomData }
    }
}

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
            LoadState::Failed => warn!(
                "Couldn't load asset {:?} of type {:?} for entity {:?}.",
                asset,
                type_name::<T>(),
                entity
            ),
            _ => {}
        }
    }
}
