use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Reflect)]
#[reflect(Component, Default)]
pub struct PointLight2d {
    /// The color of this light source.
    pub color: Color,
    /// Luminous power in lumens, representing the amount of light emitted by this source in all directions.
    pub intensity: f32,
    /// Cut-off for the light's area-of-effect. Fragments outside this range will not be affected by
    /// this light at all, so it's important to tune this together with `intensity` to prevent hard
    /// lighting cut-offs.
    pub range: f32,
    /// Simulates a light source coming from a spherical volume with the given radius. Only affects
    /// the size of specular highlights created by this light. Because of this, large values may not
    /// produce the intended result -- for example, light radius does not affect shadow softness or
    /// diffuse lighting.
    pub radius: f32,
}

impl Default for PointLight2d {
    fn default() -> Self {
        PointLight2d {
            color: Color::WHITE,
            // 1,000,000 lumens is a very large "cinema light" capable of registering brightly at Bevy's
            // default "very overcast day" exposure level. For "indoor lighting" with a lower exposure,
            // this would be way too bright.
            intensity: 1_000_000.0,
            range: 20.0,
            radius: 0.0,
        }
    }
}

#[derive(Bundle, Default)]
pub struct PointLight2dBundle {
    pub point_light: PointLight2d,
    /// The visibility of the entity.
    pub visibility: Visibility,
    /// The inherited visibility of the entity.
    pub inherited_visibility: InheritedVisibility,
    /// The view visibility of the entity.
    pub view_visibility: ViewVisibility,
    /// The transform of the entity.
    pub transform: Transform,
    /// The global transform of the entity.
    pub global_transform: GlobalTransform,
}
