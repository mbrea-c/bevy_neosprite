use crate::SpriteAtlasLoaderError;
use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    color::Color,
    render::texture::ImageLoaderSettings,
};
use serde::{Deserialize, Serialize};

use super::NeoMaterial;

#[derive(Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct NeoMaterialSerial {
    #[serde(default)]
    pub color: Color,
    #[serde(default)]
    pub texture: Option<String>,
    #[serde(default)]
    pub normal: Option<String>,
}

#[derive(Default)]
pub struct NeoMaterialAssetLoader;

impl AssetLoader for NeoMaterialAssetLoader {
    type Asset = NeoMaterial;
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
        let sprite_atlas_serial = ron::de::from_bytes::<NeoMaterialSerial>(&bytes)?;
        let texture = sprite_atlas_serial.texture.as_ref().map(|p| lc.load(p));
        let normal = sprite_atlas_serial.normal.as_ref().map(|p| {
            lc.loader()
                .with_settings(|settings: &mut ImageLoaderSettings| settings.is_srgb = false)
                .load(p)
        });

        Ok(NeoMaterial {
            color: sprite_atlas_serial.color,
            texture,
            normal,
        })
    }

    fn extensions(&self) -> &[&str] {
        &[".neomat.ron"]
    }
}
