#import bevy_sprite::{
    mesh2d_vertex_output::VertexOutput,
    mesh2d_view_bindings::{view, globals},
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

@group(2) @binding(0) var<uniform> material: ColorMaterial;
@group(2) @binding(1) var texture: texture_2d<f32>;
@group(2) @binding(2) var texture_sampler: sampler;
@group(2) @binding(3) var normal: texture_2d<f32>;
@group(2) @binding(4) var normal_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    var base_color = material.color;
    var rendered_color: vec3<f32> = vec3(0.,0.,0.);

    // TODO: CURRNETLY DOES NOTHING. WANT TO CHANGE?
 //#ifdef VERTEX_COLORS
 //    output_color = output_color * mesh.color;
 //#endif

    // First we get the location of the current fragment relative to camera
    let camera_pos = vec3<f32>(0., 0., 0.);
    let fragment_pos = mesh.world_position.xyz;

    if ((material.flags & NEO_MATERIAL_FLAGS_TEXTURE_BIT) != 0u) {
        base_color = textureSample(texture, texture_sampler, mesh.uv);
    }

    if base_color.a < 0.1 {
        discard;
    }

    rendered_color = rendered_color + base_color.rgb * AMBIENT;

    if ((material.flags & NEO_MATERIAL_FLAGS_NORMAL_BIT) != 0u) {
        let normal_color = textureSample(normal, normal_sampler, mesh.uv);
        let normal_vec = normalize(normal_color.xyz * 2. - 1.);
        let light_pos = vec3(0., 0., 2.5);
        let light_vec = normalize(light_pos - fragment_pos);

        let diffuse_intensity = max(dot(normal_vec, light_vec), 0.);

        rendered_color = rendered_color + diffuse_intensity * base_color.rgb * 5.;
    }
    var output_color = vec4(rendered_color, base_color.a);
#ifdef TONEMAP_IN_SHADER
    output_color = tonemapping::tone_mapping(output_color, view.color_grading);
#endif
    return output_color;
}
