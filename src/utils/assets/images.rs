use bevy::{prelude::*, utils::HashMap};

use crate::game::card::{Card, CARD_BACK_PATH};

use super::{AssetKey, HandleMap};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum ImageKey {
    Card(Card),
    CardBack,
}

impl AssetKey for ImageKey {
    type Asset = Image;
}

impl FromWorld for HandleMap<ImageKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        let cards: HashMap<ImageKey, Handle<<ImageKey as AssetKey>::Asset>> =
            Card::all_variations()
                .into_iter()
                .map(|card| (ImageKey::Card(card), asset_server.load(card.texture_path())))
                .collect();
        let mut images: HashMap<ImageKey, Handle<<ImageKey as AssetKey>::Asset>> =
            [(ImageKey::CardBack, asset_server.load(CARD_BACK_PATH))].into();

        images.extend(cards);
        images.into()
    }
}
