use bevy::{
    color::palettes::css::{BLUE, GREEN, RED, WHITE},
    prelude::*,
    render::texture::ImageLoaderSettings,
};
use bevy_neosprite::{
    NeoMaterial, NeoMesh2dBundle, PointLight2d, PointLight2dBundle, SpritePlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SpritePlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, update_cursor_aim)
        .run();
}

#[derive(Component)]
pub struct Aaaaa;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<NeoMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(NeoMesh2dBundle {
        mesh: meshes.add(Circle { radius: 48. }).into(),
        material: materials.add(Color::from(WHITE)),
        ..default()
    });
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
    commands
        .spawn(NeoMesh2dBundle {
            mesh: meshes.add(Rectangle::new(512., 512.)).into(),
            transform: Transform::default(),
            material: materials.add(NeoMaterial {
                color: WHITE.into(),
                texture: Some(asset_server.load("sprites/diffuse_0000.png")),
                normal: Some(asset_server.load_with_settings(
                    "sprites/normal_0000.png",
                    |settings: &mut ImageLoaderSettings| settings.is_srgb = false,
                )),
            }),
            ..default()
        })
        .insert(Aaaaa);
}

pub fn update_cursor_aim(
    mut players: Query<(&mut Transform, &Handle<NeoMaterial>), With<Aaaaa>>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
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
    }
}
