pub(crate) mod images;
pub(crate) mod loader;

use bevy::{prelude::*, utils::HashMap};

// TODO: check if this is really useful for tests, or could be rewritten in a better way
#[cfg(test)]
pub use loader::is_asset_loaded;
pub use loader::Loaded;

pub(super) fn plugin(app: &mut App) {
    use self::images::ImageKey;

    app.register_type::<HandleMap<ImageKey>>();
    app.init_resource::<HandleMap<ImageKey>>();
}

pub trait AssetKey: Sized {
    type Asset: Asset;
}

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct HandleMap<K: AssetKey>(HashMap<K, Handle<K::Asset>>);

impl<K: AssetKey, T> From<T> for HandleMap<K>
where
    T: Into<HashMap<K, Handle<K::Asset>>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<K: AssetKey> HandleMap<K> {
    #[cfg(test)]
    pub fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}
