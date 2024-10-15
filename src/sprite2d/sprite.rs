use bevy::{
    math::Vec2,
    prelude::{Commands, Component, Entity, Query},
};

use crate::Mesh2dUvRange;

#[derive(Component)]
pub struct Sprite2d {
    /// Number of sprites in each row
    pub sprites_horizontal: u32,
    /// Number of sprites in each column
    pub sprites_vertical: u32,
    /// Total sprites (may not equal to horizontal * vertical if the last row is not full)
    pub sprites_total: u32,

    /// Current visible sprite
    pub sprite_index: u32,
}

pub fn update_sprite_2d_uv_ranges(mut commands: Commands, q_sprites: Query<(Entity, &Sprite2d)>) {
    for (entity, sprite) in &q_sprites {
        let x_idx = sprite.sprite_index % sprite.sprites_horizontal;
        let y_idx = sprite.sprite_index / sprite.sprites_horizontal;

        let x_size = 1. / sprite.sprites_horizontal as f32;
        let y_size = 1. / sprite.sprites_vertical as f32;

        let uv_range = Mesh2dUvRange {
            start: Vec2::new(x_idx as f32 * x_size, y_idx as f32 * y_size),
            end: Vec2::new((x_idx + 1) as f32 * x_size, (y_idx + 1) as f32 * y_size),
        };

        commands.entity(entity).insert(uv_range);
    }
}
