use bevy::{asset::LoadState, prelude::*};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum TestAssetLoadingState {
    #[default]
    Loading,
    Loaded,
}

/// checks that all Assets of type T are loaded before setting state
pub fn are_all_images_loaded(
    handles_query: Query<&Sprite>,
    assets: Res<AssetServer>,
    mut loading_state: ResMut<NextState<TestAssetLoadingState>>,
) {
    if handles_query
        .iter()
        .all(|sprite| assets.load_state(sprite.image.id()).is_loaded())
    {
        loading_state.set(TestAssetLoadingState::Loaded);
    }
}

#[derive(Component)]
pub struct ImageLoaded;

#[cfg(test)]
pub fn is_image_loaded(
    assets_query: Query<(Entity, &Sprite), Without<ImageLoaded>>,
    assets: Res<AssetServer>,
    mut commands: Commands,
) {
    for (entity, sprite) in assets_query.iter() {
        let load_state = assets.load_state(sprite.image.id());
        match load_state {
            LoadState::Loaded => {
                println!(
                    "Finished loading asset {:?} for entity {:?}.",
                    sprite, entity
                );
                commands.entity(entity).insert(ImageLoaded);
            }
            LoadState::Failed(err) => println!(
                "Couldn't load asset {:?} for entity {:?} with error: {:?}",
                sprite, entity, err,
            ),
            _ => {}
        }
    }
}
