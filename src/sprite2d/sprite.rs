use crate::{Material2d, Mesh2dUvRange};
use bevy::{
    asset::{Asset, Assets, Handle},
    math::Vec2,
    prelude::{Commands, Component, Entity, Query, Res},
    reflect::Reflect,
};
use serde::{Deserialize, Serialize};

pub struct SpriteAtlasHandle<T: Material2d>(pub Handle<SpriteAtlas<T>>);

#[derive(Asset, Reflect, Clone, PartialEq)]
pub struct SpriteAtlas<T: Material2d> {
    /// Animations in this Atlas
    pub animations: Vec<SpriteAnimation>,
    pub material: Handle<T>,
}

#[derive(Reflect, Serialize, Deserialize, Clone, PartialEq)]
pub enum SpriteAnimation {
    UniformGrid {
        /// UV coordinates of the start of the region containing this animation's sprites (top left)
        region_start: (f32, f32),
        /// UV coordinates of the end of the region containing this animation's sprites (bottom right)
        region_end: (f32, f32),
        /// Number of sprites in each row
        sprites_horizontal: u32,
        /// Number of sprites in each column
        sprites_vertical: u32,
        /// Total sprites (may not equal to horizontal * vertical if the last row is not full)
        sprites_total: u32,
    },
}

#[derive(Component, Reflect, Serialize, Deserialize, Clone, PartialEq)]
pub struct ActiveSprite {
    pub animation: usize,
    pub frame: u32,
}

pub fn update_sprite_atlas_material<T: Material2d>(
    mut commands: Commands,
    q_sprites: Query<(Entity, &Handle<SpriteAtlas<T>>, Option<&Handle<T>>)>,
    a_sprite_atlas: Res<Assets<SpriteAtlas<T>>>,
) {
    for (entity, atlas_handle, maybe_material_handle) in &q_sprites {
        let Some(atlas) = a_sprite_atlas.get(atlas_handle) else {
            continue;
        };

        let should_insert = maybe_material_handle.map_or(true, |material_handle| {
            material_handle.id() != atlas.material.id()
        });

        if should_insert {
            commands.entity(entity).insert(atlas.material.clone());
        }
    }
}

pub fn update_sprite_atlas_uv_ranges<T: Material2d>(
    mut commands: Commands,
    q_sprites: Query<(Entity, &Handle<SpriteAtlas<T>>, &ActiveSprite)>,
    a_sprite_atlas: Res<Assets<SpriteAtlas<T>>>,
) {
    for (entity, atlas_handle, active_sprite) in &q_sprites {
        let Some(atlas) = a_sprite_atlas.get(atlas_handle) else {
            continue;
        };

        match atlas.animations[active_sprite.animation] {
            SpriteAnimation::UniformGrid {
                region_start,
                region_end,
                sprites_horizontal,
                sprites_vertical,
                ..
            } => {
                let x_idx = active_sprite.frame % sprites_horizontal;
                let y_idx = active_sprite.frame / sprites_horizontal;

                let range_x = region_end.0 - region_start.0;
                let range_y = region_end.1 - region_start.1;

                let x_size = range_x / sprites_horizontal as f32;
                let y_size = range_y / sprites_vertical as f32;

                let uv_range = Mesh2dUvRange {
                    start: Vec2::new(
                        region_start.0 + x_idx as f32 * x_size,
                        region_start.1 + y_idx as f32 * y_size,
                    ),
                    end: Vec2::new(
                        region_start.0 + (x_idx + 1) as f32 * x_size,
                        region_start.1 + (y_idx + 1) as f32 * y_size,
                    ),
                };

                commands.entity(entity).insert(uv_range);
            }
        }
    }
}
