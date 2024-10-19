#import bevy_sprite::{
    mesh2d_vertex_output::VertexOutput,
    mesh2d_view_bindings::{view, lights},
    mesh2d_functions as fns,
}

#ifdef TONEMAP_IN_SHADER
#import bevy_core_pipeline::tonemapping
#endif

struct ColorMaterial {
    color: vec4<f32>,
    // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    flags: u32,
};
const NEO_MATERIAL_FLAGS_TEXTURE_BIT: u32 = 1u;
const NEO_MATERIAL_FLAGS_NORMAL_BIT: u32 = 2u;
const AMBIENT: f32 = 0.1;
const BIAS: f32 = 1.;

@group(2) @binding(0) var<uniform> material: ColorMaterial;
@group(2) @binding(1) var texture: texture_2d<f32>;
@group(2) @binding(2) var texture_sampler: sampler;
@group(2) @binding(3) var normal_texture: texture_2d<f32>;
@group(2) @binding(4) var normal_texture_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    var base_color = material.color;
    var rendered_color: vec3<f32> = vec3(0.,0.,0.);
    let uv = fns::map_uv_range(mesh.uv, mesh.index);

    let world_pos = mesh.world_position.xyz;

    if ((material.flags & NEO_MATERIAL_FLAGS_TEXTURE_BIT) != 0u) {
        base_color = textureSample(texture, texture_sampler, uv);
    }

    if base_color.a < 0.1 {
        discard;
    }

    rendered_color = rendered_color + base_color.rgb * AMBIENT;

    if ((material.flags & NEO_MATERIAL_FLAGS_NORMAL_BIT) != 0u) {
        let normal_color = textureSample(normal_texture, normal_texture_sampler, uv);
        let normal_vec = normalize(normal_color.xyz * 2. - 1.);
        let world_normal = fns::mesh2d_normal_local_to_world(normal_vec, mesh.index);

        for (var i = 0u; i < lights.n_point_lights; i++) {
            let light = lights.point_lights[i];
            let light_pos = light.pos;
            let light_vec = light_pos - world_pos;
            let light_vec_normalized = normalize(light_vec);
            let diffuse_intensity = max(dot(world_normal, light_vec_normalized), 0.);
            let attenuation_factor = 1. / (BIAS + length(light_vec) * length(light_vec));

            rendered_color = rendered_color + diffuse_intensity * base_color.rgb * light.color.rgb * attenuation_factor;
        }
    }
    var output_color = vec4(rendered_color, base_color.a);
#ifdef TONEMAP_IN_SHADER
    output_color = tonemapping::tone_mapping(output_color, view.color_grading);
#endif
    return output_color;
}
