use bevy::{prelude::*, render::pass::ClearColor, window::CursorMoved};
use std::{convert::TryInto, net::UdpSocket};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SENDER: UdpSocket =
        UdpSocket::bind("127.0.0.1:8080").expect("couldn't bind to address");
}
fn main() {
    App::build()
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_system(networked_ball_movement_system.system())
        .run();
}
struct Ball {
    velocity: Vec3,
}
fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // Add the game's entities to our world

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    // ball
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            transform: Transform::from_xyz(0.0, -50.0, 1.0),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .insert(Ball {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
        });
}
fn networked_ball_movement_system(
    time: Res<Time>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut ball_query: Query<(&Ball, &mut Transform)>,
    wnds: Res<Windows>,
) {
    // get the primary window
    let wnd = wnds.get_primary().unwrap();

    // Get message
    let (x, y) = get_networked_message().unwrap();
    println!("x:{:?}, y:{:?}", x, y);

    if let Ok((ball, mut transform)) = ball_query.single_mut() {
        transform.translation.x = x;
        transform.translation.y = y;
        println!("x:{:?}, y:{:?}", x, y);
    }
}
fn get_networked_message() -> std::io::Result<(f32, f32)> {
    let mut buf = [0; 8];
    let (amt, src) = SENDER.recv_from(&mut buf)?;
    let buf = &mut buf[..amt];
    let x = f32::from_be_bytes(buf[0..4].try_into().unwrap());
    let y = f32::from_be_bytes(buf[4..8].try_into().unwrap());
    println!("1 x:{:?}, y:{:?}", x, y);
    Ok((x, y))
}
