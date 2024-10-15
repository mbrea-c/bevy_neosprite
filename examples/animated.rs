use std::time::Duration;

use bevy::{
    color::palettes::css::{BLUE, GREEN, RED, WHITE},
    prelude::*,
    render::texture::ImageLoaderSettings,
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
        .add_systems(Startup, setup)
        .add_systems(Update, (update_cursor_aim, animate))
        .run();
}

#[derive(Component)]
pub struct Aaaaa;

#[derive(Component)]
pub struct FrameTimer(Timer);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<NeoMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(PointLight2dBundle {
        point_light: PointLight2d {
            color: RED.into(),
            intensity: 5.,
            range: 5.,
            radius: 1.,
        },
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
    commands.spawn(PointLight2dBundle {
        point_light: PointLight2d {
            color: GREEN.into(),
            intensity: 5.,
            range: 5.,
            radius: 1.,
        },
        transform: Transform::from_xyz(-500., 0., 0.),
        ..default()
    });
    commands.spawn(PointLight2dBundle {
        point_light: PointLight2d {
            color: BLUE.into(),
            intensity: 5.,
            range: 5.,
            radius: 1.,
        },
        transform: Transform::from_xyz(0., 1000., 0.),
        ..default()
    });
    let id = commands
        .spawn(NeoMesh2dBundle {
            mesh: meshes.add(Rectangle::new(512., 512.)).into(),
            transform: Transform::default(),
            material: materials.add(NeoMaterial {
                color: WHITE.into(),
                texture: Some(asset_server.load("sprites/spritesheet_diffuse.png")),
                normal: Some(asset_server.load_with_settings(
                    "sprites/spritesheet_normal.png",
                    |settings: &mut ImageLoaderSettings| settings.is_srgb = false,
                )),
            }),
            ..default()
        })
        .insert(Sprite2d {
            sprites_horizontal: 10,
            sprites_vertical: 4,
            sprites_total: 9,
            sprite_index: 0,
        })
        .insert(FrameTimer(Timer::new(
            Duration::from_secs_f32(0.15),
            TimerMode::Repeating,
        )))
        .insert(Aaaaa)
        .id();
    println!("Created character as id: {:?}", id);
}

fn update_cursor_aim(
    mut players: Query<(&mut Transform, &Handle<NeoMaterial>), With<Aaaaa>>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    time: Res<Time>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let Ok((camera, camera_transform)) = q_camera.get_single() else {
        warn!("Cannot get mouse pos, there isn't a single camera!");
        return;
    };

    for (mut transform, _) in &mut players {
        if let Ok(window) = q_window.get_single() {
            if let Some(world_position) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                .map(|ray| ray.origin.truncate())
            {
                transform.translation = Vec3::new(world_position.x, world_position.y, 0.);
            }
        }

        transform.rotation = Quat::from_rotation_z(1. * time.delta_seconds()) * transform.rotation;
    }
}

fn animate(mut q_sprite: Query<(&mut Sprite2d, &mut FrameTimer)>, time: Res<Time>) {
    for (mut sprite, mut timer) in &mut q_sprite {
        if timer.0.tick(time.delta()).just_finished() {
            sprite.sprite_index = (sprite.sprite_index + 1) % (sprite.sprites_total - 1);
        }
    }
}
