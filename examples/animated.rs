fn main() {}
// use std::time::Duration;
//
// use bevy::{
//     color::palettes::css::{ORANGE, WHITE},
//     prelude::*,
//     render::{camera::ScalingMode, texture::ImageLoaderSettings},
// };
// use bevy_neosprite::{
//     NeoMaterial, NeoMesh2dBundle, PointLight2d, PointLight2dBundle, SpriteAtlas, SpritePlugin,
// };
//
// fn main() {
//     App::new()
//         .add_plugins((
//             DefaultPlugins.set(ImagePlugin::default_nearest()),
//             SpritePlugin,
//         ))
//         .add_systems(Startup, (setup, setup_floor))
//         .add_systems(Update, (update_cursor_aim, animate))
//         .run();
// }
//
// #[derive(Component)]
// pub struct Aaaaa;
//
// #[derive(Component)]
// pub struct FrameTimer(Timer);
//
// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<NeoMaterial>>,
//     asset_server: Res<AssetServer>,
// ) {
//     commands.spawn(Camera2dBundle {
//         projection: OrthographicProjection {
//             scaling_mode: ScalingMode::FixedVertical(40.),
//             near: -1000.0,
//             far: 1000.0,
//             ..default()
//         },
//
//         ..default()
//     });
//     commands.spawn(PointLight2dBundle {
//         point_light: PointLight2d {
//             color: ORANGE.into(),
//             intensity: 5000.,
//             range: 5.,
//             radius: 1.,
//         },
//         transform: Transform::from_xyz(0., 0., 3.),
//         ..default()
//     });
//     commands
//         .spawn((SpatialBundle::default(), Aaaaa))
//         .with_children(|c| {
//             c.spawn(NeoMesh2dBundle {
//                 mesh: meshes.add(Rectangle::new(4., 4.)).into(),
//                 transform: Transform::from_xyz(0., -0.2, 0.),
//                 material: materials.add(NeoMaterial {
//                     color: WHITE.into(),
//                     texture: Some(asset_server.load("sprites/spritesheet_diffuse.png")),
//                     normal: Some(asset_server.load_with_settings(
//                         "sprites/spritesheet_normal.png",
//                         |settings: &mut ImageLoaderSettings| settings.is_srgb = false,
//                     )),
//                 }),
//                 ..default()
//             })
//             .insert(Sprite2d {
//                 sprites_horizontal: 10,
//                 sprites_vertical: 4,
//                 sprites_total: 9,
//                 sprite_index: 0,
//             })
//             .insert(FrameTimer(Timer::new(
//                 Duration::from_secs_f32(0.15),
//                 TimerMode::Repeating,
//             )));
//         });
// }
//
// fn setup_floor(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     asset_server: Res<AssetServer>,
// ) {
//     let n = 30;
//     let size = 8.;
//     for x in -n..n {
//         for y in -n..n {
//             commands
//                 .spawn(NeoMesh2dBundle {
//                     mesh: meshes.add(Rectangle::new(size, size)).into(),
//                     transform: Transform::from_xyz(x as f32 * size, y as f32 * size, 0.),
//                     ..default()
//                 })
//                 .insert(
//                     asset_server.load::<SpriteAtlas<NeoMaterial>>("sprites/cobblestone.atlas.ron"),
//                 );
//         }
//     }
// }
//
// fn update_cursor_aim(
//     mut players: Query<&mut Transform, With<Aaaaa>>,
//     q_window: Query<&Window>,
//     q_camera: Query<(&Camera, &GlobalTransform)>,
//     time: Res<Time>,
// ) {
//     // get the camera info and transform
//     // assuming there is exactly one main camera entity, so Query::single() is OK
//     let Ok((camera, camera_transform)) = q_camera.get_single() else {
//         warn!("Cannot get mouse pos, there isn't a single camera!");
//         return;
//     };
//
//     for mut transform in &mut players {
//         if let Ok(window) = q_window.get_single() {
//             if let Some(world_position) = window
//                 .cursor_position()
//                 .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
//                 .map(|ray| ray.origin.truncate())
//             {
//                 let cur_pos = transform.translation.xy();
//                 let delta =
//                     (world_position - cur_pos).normalize_or_zero() * time.delta_seconds() * 3.;
//                 let new_pos = if delta.length() >= world_position.distance(cur_pos) {
//                     world_position
//                 } else {
//                     cur_pos + delta
//                 };
//                 transform.translation = Vec3::new(new_pos.x, new_pos.y, 0.);
//                 if delta != Vec2::ZERO {
//                     transform.rotation = Quat::from_rotation_arc_2d(-Vec2::Y, delta.normalize());
//                 }
//             }
//         }
//     }
// }
//
// fn animate(mut q_sprite: Query<(&mut Sprite2d, &mut FrameTimer)>, time: Res<Time>) {
//     for (mut sprite, mut timer) in &mut q_sprite {
//         if timer.0.tick(time.delta()).just_finished() {
//             sprite.sprite_index = (sprite.sprite_index + 1) % (sprite.sprites_total - 1);
//         }
//     }
// }
