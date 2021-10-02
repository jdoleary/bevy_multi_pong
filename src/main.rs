use bevy::{
    // input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
    render::pass::ClearColor,
    window::CursorMoved,
};
fn main() {
    App::build()
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_system(ball_movement_system.system())
        .run();
}
struct Ball {
    velocity: Vec3,
}
fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
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
fn ball_movement_system(
    time: Res<Time>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut ball_query: Query<(&Ball, &mut Transform)>,
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    // q_camera: Query<&Transform, With<MainCamera>>,
) {
    // clamp the timestep to stop the ball from escaping when the game starts
    let delta_seconds = f32::min(0.2, time.delta_seconds());

    // for event in cursor_moved_events.iter() {
    //     println!("{:?}", event);
    //     if let Ok((ball, mut transform)) = ball_query.single_mut() {
    //         transform.translation.x = event.position.x;
    //         transform.translation.y = event.position.y;
    //     }
    // }

    // get the primary window
    let wnd = wnds.get_primary().unwrap();

    // check if the cursor is in the primary window
    if let Some(pos) = wnd.cursor_position() {
        // get the size of the window
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = pos - size / 2.0;

        // // assuming there is exactly one main camera entity, so this is OK
        // let camera_transform = q_camera.single().unwrap();

        // // apply the camera transform
        // let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        eprintln!("World coords: {:?}", p);
        if let Ok((ball, mut transform)) = ball_query.single_mut() {
            transform.translation.x = p.x;
            transform.translation.y = p.y;
        }
    }
}
