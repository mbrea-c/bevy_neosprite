use super::{SpriteAnimation, SpriteAtlas};
use crate::{Material2d, SpriteAtlasLoaderError};
use bevy::asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct SpriteAtlasSerial {
    /// Animations in this Atlas
    pub animations: Vec<SpriteAnimation>,
    pub material_path: String,
}

#[derive(Default)]
pub struct SpriteAtlasAssetLoader<T: Material2d>(PhantomData<T>);

impl<T: Material2d> AssetLoader for SpriteAtlasAssetLoader<T> {
    type Asset = SpriteAtlas<T>;
    type Settings = ();
    type Error = SpriteAtlasLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        lc: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let sprite_atlas_serial = ron::de::from_bytes::<SpriteAtlasSerial>(&bytes)?;
        let material = lc.load(&sprite_atlas_serial.material_path);

        Ok(SpriteAtlas {
            animations: sprite_atlas_serial.animations,
            material,
        })
    }

    fn extensions(&self) -> &[&str] {
        &[".atlas.ron"]
    }
}
