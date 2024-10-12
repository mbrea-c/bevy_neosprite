#define_import_path bevy_sprite::mesh2d_view_types

#import bevy_render::view
#import bevy_render::globals

struct Lights {
    // NOTE: this array size must be kept in sync with the constants defined in bevy_pbr/src/render/light.rs
    point_lights: array<PointLight, 32u>,
    ambient_color: vec3<f32>,
    n_point_lights: u32,
};

struct PointLight {
    color: vec4<f32>,
    pos: vec3<f32>,
    range: f32,
    radius: f32,
    flags: u32,
};

