use std::time::Duration;

use bevy::{
    color::palettes::css::{ORANGE, WHITE},
    prelude::*,
    render::{camera::ScalingMode, texture::ImageLoaderSettings},
};
use bevy_neosprite::{
    NeoMaterial, NeoMesh2dBundle, PointLight2d, PointLight2dBundle, Sprite2d, SpritePlugin,
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<NeoMaterial>>,
    asset_server: Res<AssetServer>,
) {
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
                material: materials.add(NeoMaterial {
                    color: WHITE.into(),
                    texture: Some(asset_server.load("sprites/monkey/spritesheet_diffuse.png")),
                    normal: Some(asset_server.load_with_settings(
                        "sprites/monkey/spritesheet_normal.png",
                        |settings: &mut ImageLoaderSettings| settings.is_srgb = false,
                    )),
                }),
                ..default()
            })
            .insert(Sprite2d {
                sprites_horizontal: 10,
                sprites_vertical: 3,
                sprites_total: 23,
                sprite_index: 0,
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
    mut materials: ResMut<Assets<NeoMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let n = 30;
    let size = 8.;
    for x in -n..n {
        for y in -n..n {
            commands.spawn(NeoMesh2dBundle {
                mesh: meshes.add(Rectangle::new(size, size)).into(),
                transform: Transform::from_xyz(x as f32 * size, y as f32 * size, 0.),
                material: materials.add(NeoMaterial {
                    color: WHITE.into(),
                    texture: Some(asset_server.load("sprites/cobblestone/diffuse.png")),
                    normal: Some(asset_server.load_with_settings(
                        "sprites/cobblestone/normal.png",
                        |settings: &mut ImageLoaderSettings| settings.is_srgb = false,
                    )),
                }),
                ..default()
            });
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

fn animate(mut q_sprite: Query<(&mut Sprite2d, &mut FrameTimer)>, time: Res<Time>) {
    for (mut sprite, mut timer) in &mut q_sprite {
        if timer.0.tick(time.delta()).just_finished() {
            sprite.sprite_index = (sprite.sprite_index + 1) % sprite.sprites_total;
        }
    }
}
