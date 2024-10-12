use bevy::{
    prelude::*,
    render::{
        render_resource::{DynamicUniformBuffer, ShaderType},
        renderer::{RenderDevice, RenderQueue},
        texture::TextureCache,
        view::{ExtractedView, RenderLayers},
        Extract,
    },
};

use super::PointLight2d;

#[derive(Component, Debug)]
pub struct ExtractedPointLight {
    pub color: LinearRgba,
    /// luminous intensity in lumens per steradian
    pub intensity: f32,
    pub range: f32,
    pub radius: f32,
    pub transform: GlobalTransform,
}

pub const MAX_POINT_LIGHTS: usize = 32;

#[derive(Copy, Clone, ShaderType, Default, Debug)]
pub struct GpuPointLight {
    color: Vec4,
    pos: Vec3,
    range: f32,
    radius: f32,
    flags: u32,
}

#[derive(Copy, Clone, Debug, ShaderType)]
pub struct GpuLights {
    point_lights: [GpuPointLight; MAX_POINT_LIGHTS],
    ambient_color: Vec3,
    n_point_lights: u32,
}

#[derive(Component)]
pub struct ViewLightEntities {
    pub lights: Vec<Entity>,
}

#[derive(Component)]
pub struct ViewLightsUniformOffset {
    pub offset: u32,
}

#[derive(Resource, Default)]
pub struct LightMeta {
    pub view_gpu_lights: DynamicUniformBuffer<GpuLights>,
}

#[allow(clippy::too_many_arguments)]
pub fn extract_lights(
    mut commands: Commands,
    point_lights: Extract<Query<(Entity, &PointLight2d, &GlobalTransform, &ViewVisibility)>>,
    mut previous_point_lights_len: Local<usize>,
) {
    let mut point_lights_values = Vec::with_capacity(*previous_point_lights_len);
    println!("Extract system is running");
    for (entity, point_light, transform, view_visibility) in &point_lights {
        // if !view_visibility.get() {
        //     println!("Not visible, skipping...");
        //     continue;
        // }
        let extracted_point_light = ExtractedPointLight {
            color: point_light.color.into(),
            // NOTE: Map from luminous power in lumens to luminous intensity in lumens per steradian
            // for a point light. See https://google.github.io/filament/Filament.html#mjx-eqn-pointLightLuminousPower
            // for details.
            intensity: point_light.intensity / (4.0 * std::f32::consts::PI),
            range: point_light.range,
            radius: point_light.radius,
            transform: *transform,
        };

        println!("Extracted light: {:#?}", extracted_point_light);

        point_lights_values.push((entity, (extracted_point_light,)));
    }
    *previous_point_lights_len = point_lights_values.len();
    commands.insert_or_spawn_batch(point_lights_values);
}

#[allow(clippy::too_many_arguments)]
pub fn prepare_lights(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    mut light_meta: ResMut<LightMeta>,
    views: Query<Entity, (With<Camera2d>, With<ExtractedView>)>,
    // ambient_light: Res<AmbientLight>,
    point_lights: Query<(Entity, &ExtractedPointLight)>,
) {
    let views_iter = views.iter();
    let views_count = views_iter.len();
    let Some(mut view_gpu_lights_writer) =
        light_meta
            .view_gpu_lights
            .get_writer(views_count, &render_device, &render_queue)
    else {
        return;
    };

    let point_lights: Vec<_> = point_lights.iter().collect::<Vec<_>>();

    #[cfg(any(
        not(feature = "webgl"),
        not(target_arch = "wasm32"),
        feature = "webgpu"
    ))]
    #[cfg(any(
        not(feature = "webgl"),
        not(target_arch = "wasm32"),
        feature = "webgpu"
    ))]
    let mut gpu_point_lights = [GpuPointLight::default(); MAX_POINT_LIGHTS];
    for (index, (_light_entity, light)) in point_lights.iter().enumerate().take(MAX_POINT_LIGHTS) {
        gpu_point_lights[index] = GpuPointLight {
            color: light.color.to_vec4(),
            pos: light.transform.translation(),
            range: light.range,
            radius: light.radius,
            flags: 0,
        };
    }

    for view_entity in views.iter() {
        let view_lights = Vec::new();

        let ambient_light = AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        };

        let gpu_lights = GpuLights {
            point_lights: gpu_point_lights,
            ambient_color: Vec4::from_slice(&LinearRgba::from(ambient_light.color).to_f32_array())
                .xyz()
                * ambient_light.brightness,
            n_point_lights: point_lights.iter().len().min(MAX_POINT_LIGHTS) as u32,
        };

        commands.entity(view_entity).insert((
            ViewLightEntities {
                lights: view_lights,
            },
            ViewLightsUniformOffset {
                offset: view_gpu_lights_writer.write(&gpu_lights),
            },
        ));
    }
}
