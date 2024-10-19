// FIXME(3492): remove once docs are ready
#![allow(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://bevyengine.org/assets/icon.png",
    html_favicon_url = "https://bevyengine.org/assets/icon.png"
)]

//! Provides 2D sprite rendering functionality.
mod mesh2d;
mod sprite2d;
mod utils;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::{NeoMaterial, NeoMesh2dBundle};
}

use bevy::render::render_asset::prepare_assets;
use bevy::render::texture::GpuImage;
use bevy::render::{ExtractSchedule, Render, RenderApp, RenderSet};
pub use mesh2d::*;
pub use sprite2d::*;
pub use utils::error::*;

use bevy::app::prelude::*;
use bevy::asset::{AssetApp, Assets, Handle};
use bevy::ecs::prelude::*;
use bevy::render::{
    mesh::Mesh,
    primitives::Aabb,
    render_resource::Shader,
    view::{check_visibility, NoFrustumCulling, VisibilitySystems},
};

/// Adds support for 2D sprite rendering.
#[derive(Default)]
pub struct SpritePlugin;

pub const SPRITE_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(2763343953151597127);
pub const SPRITE_VIEW_BINDINGS_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(8846920112458963210);

/// System set for sprite rendering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum SpriteSystem {
    ExtractSprites,
    ComputeSlices,
}

/// A convenient alias for `With<Mesh2dHandle>>`, for use with
/// [`bevy_render::view::VisibleEntities`].
pub type WithMesh2d = With<Mesh2dHandle>;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SpriteAtlas<NeoMaterial>>()
            .init_asset_loader::<SpriteAtlasAssetLoader<NeoMaterial>>();

        app.register_type::<Mesh2dHandle>()
            .add_plugins((NeoMesh2dRenderPlugin, NeoMaterialPlugin))
            .add_systems(
                PostUpdate,
                (
                    calculate_bounds_2d.in_set(VisibilitySystems::CalculateBounds),
                    (check_visibility::<WithMesh2d>,).in_set(VisibilitySystems::CheckVisibility),
                    //update_sprite_2d_uv_ranges,
                    update_sprite_atlas_material::<NeoMaterial>,
                    update_sprite_atlas_uv_ranges::<NeoMaterial>,
                ),
            );

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app
            .add_systems(ExtractSchedule, extract_lights)
            .add_systems(
                Render,
                (prepare_lights
                    .in_set(RenderSet::ManageViews)
                    .after(prepare_assets::<GpuImage>),),
            )
            .init_resource::<LightMeta>();
    }
}

/// System calculating and inserting an [`Aabb`] component to entities with either:
/// - a `Mesh2dHandle` component,
///   and without a [`NoFrustumCulling`] component.
///
/// Used in system set [`VisibilitySystems::CalculateBounds`].
#[allow(clippy::type_complexity)]
pub fn calculate_bounds_2d(
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    meshes_without_aabb: Query<(Entity, &Mesh2dHandle), (Without<Aabb>, Without<NoFrustumCulling>)>,
) {
    for (entity, mesh_handle) in &meshes_without_aabb {
        if let Some(mesh) = meshes.get(&mesh_handle.0) {
            if let Some(aabb) = mesh.compute_aabb() {
                commands.entity(entity).try_insert(aabb);
            }
        }
    }
}
