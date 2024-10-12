use crate::{Material2d, Material2dPlugin, MaterialMesh2dBundle};
use bevy::app::{App, Plugin};
use bevy::asset::{load_internal_asset, Asset, AssetApp, Assets, Handle};
use bevy::color::{Color, ColorToComponents, LinearRgba};
use bevy::math::Vec4;
use bevy::reflect::prelude::*;
use bevy::render::{
    render_asset::RenderAssets,
    render_resource::*,
    texture::{GpuImage, Image},
};

pub const COLOR_MATERIAL_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(3253086872234592509);

#[derive(Default)]
pub struct NeoMaterialPlugin;

impl Plugin for NeoMaterialPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            COLOR_MATERIAL_SHADER_HANDLE,
            "neo_material.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins(Material2dPlugin::<NeoMaterial>::default())
            .register_asset_reflect::<NeoMaterial>();

        app.world_mut()
            .resource_mut::<Assets<NeoMaterial>>()
            .insert(
                &Handle::<NeoMaterial>::default(),
                NeoMaterial {
                    color: Color::srgb(1.0, 0.0, 1.0),
                    ..Default::default()
                },
            );
    }
}

/// A [2d material](Material2d) that renders [2d meshes](crate::Mesh2dHandle) with a texture tinted by a uniform color
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
#[reflect(Default, Debug)]
#[uniform(0, ColorMaterialUniform)]
pub struct NeoMaterial {
    pub color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    pub normal: Option<Handle<Image>>,
}

impl NeoMaterial {
    /// Creates a new material from a given color
    pub fn from_color(color: impl Into<Color>) -> Self {
        Self::from(color.into())
    }
}

impl Default for NeoMaterial {
    fn default() -> Self {
        NeoMaterial {
            color: Color::WHITE,
            texture: None,
            normal: None,
        }
    }
}

impl From<Color> for NeoMaterial {
    fn from(color: Color) -> Self {
        NeoMaterial {
            color,
            ..Default::default()
        }
    }
}

impl From<Handle<Image>> for NeoMaterial {
    fn from(texture: Handle<Image>) -> Self {
        NeoMaterial {
            texture: Some(texture),
            ..Default::default()
        }
    }
}

// NOTE: These must match the bit flags in bevy_sprite/src/mesh2d/color_material.wgsl!
bitflags::bitflags! {
    #[repr(transparent)]
    pub struct ColorMaterialFlags: u32 {
        const TEXTURE           = 1 << 0;
        const NORMAL            = 1 << 1;
        const NONE              = 0;
        const UNINITIALIZED     = 0xFFFF;
    }
}

/// The GPU representation of the uniform data of a [`ColorMaterial`].
#[derive(Clone, Default, ShaderType)]
pub struct ColorMaterialUniform {
    pub color: Vec4,
    pub flags: u32,
}

impl AsBindGroupShaderType<ColorMaterialUniform> for NeoMaterial {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<GpuImage>) -> ColorMaterialUniform {
        let mut flags = ColorMaterialFlags::NONE;
        if self.texture.is_some() {
            flags |= ColorMaterialFlags::TEXTURE;
        }

        if self.normal.is_some() {
            flags |= ColorMaterialFlags::NORMAL;
        }

        ColorMaterialUniform {
            color: LinearRgba::from(self.color).to_f32_array().into(),
            flags: flags.bits(),
        }
    }
}

impl Material2d for NeoMaterial {
    fn fragment_shader() -> ShaderRef {
        COLOR_MATERIAL_SHADER_HANDLE.into()
    }
}

/// A component bundle for entities with a [`Mesh2dHandle`](crate::Mesh2dHandle) and a [`ColorMaterial`].
pub type NeoMesh2dBundle = MaterialMesh2dBundle<NeoMaterial>;
