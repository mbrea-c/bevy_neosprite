use std::time::Duration;

use bevy::{color::palettes::css::ORANGE, prelude::*, render::camera::ScalingMode};
use bevy_neosprite::{
    ActiveSprite, NeoMaterial, NeoMesh2dBundle, PointLight2d, PointLight2dBundle, SpriteAtlas,
    SpritePlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SpritePlugin,
        ))
        .add_systems(Startup, (setup, setup_floor))
        .add_systems(Update, (update_cursor_light, animate))
        .run();
}

#[derive(Component)]
pub struct MainObject;

#[derive(Component)]
pub struct MyLight;

#[derive(Component)]
pub struct FrameTimer(Timer);

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(40.),
            near: -1000.0,
            far: 1000.0,
            ..default()
        },

        ..default()
    });
    commands
        .spawn(PointLight2dBundle {
            point_light: PointLight2d {
                color: ORANGE.into(),
                intensity: 5000.,
                range: 5.,
                radius: 1.,
            },
            transform: Transform::from_xyz(0., 0., 3.),
            ..default()
        })
        .insert(MyLight);
    commands
        .spawn((SpatialBundle::default(), MainObject))
        .with_children(|c| {
            c.spawn(NeoMesh2dBundle {
                mesh: meshes.add(Rectangle::new(6., 6.)).into(),
                transform: Transform::from_xyz(0., 0., 0.01),
                ..default()
            })
            .insert(asset_server.load::<SpriteAtlas<NeoMaterial>>("sprites/monkey.atlas.ron"))
            .insert(ActiveSprite {
                animation: 0,
                frame: 0,
            })
            .insert(FrameTimer(Timer::new(
                Duration::from_secs_f32(0.14),
                TimerMode::Repeating,
            )));
        });
}

fn setup_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let n = 30;
    let size = 8.;
    for x in -n..n {
        for y in -n..n {
            commands
                .spawn(NeoMesh2dBundle {
                    mesh: meshes.add(Rectangle::new(size, size)).into(),
                    transform: Transform::from_xyz(x as f32 * size, y as f32 * size, 0.),
                    ..default()
                })
                .insert(
                    asset_server.load::<SpriteAtlas<NeoMaterial>>("sprites/cobblestone.atlas.ron"),
                );
        }
    }
}

fn update_cursor_light(
    mut q_lights: Query<&mut Transform, With<MyLight>>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok((camera, camera_transform)) = q_camera.get_single() else {
        warn!("Cannot get mouse pos, there isn't a single camera!");
        return;
    };

    let Ok(window) = q_window.get_single() else {
        return;
    };

    let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    else {
        return;
    };

    let Ok(mut light_transform) = q_lights.get_single_mut() else {
        return;
    };

    light_transform.translation = Vec3::new(world_position.x, world_position.y, 3.);
}

fn animate(
    mut q_sprite: Query<(
        &mut ActiveSprite,
        &Handle<SpriteAtlas<NeoMaterial>>,
        &mut FrameTimer,
    )>,
    a_sprite_atlas: Res<Assets<SpriteAtlas<NeoMaterial>>>,
    time: Res<Time>,
) {
    for (mut sprite, atlas_handle, mut timer) in &mut q_sprite {
        let Some(atlas) = a_sprite_atlas.get(atlas_handle) else {
            continue;
        };

        if timer.0.tick(time.delta()).just_finished() {
            sprite.frame = (sprite.frame + 1)
                % match atlas.animations[sprite.animation] {
                    bevy_neosprite::SpriteAnimation::UniformGrid { sprites_total, .. } => {
                        sprites_total
                    }
                };
        }
    }
}
