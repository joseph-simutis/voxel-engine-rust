use bevy::prelude::*;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, States)]
pub enum GameState {
    Menu,
    TitleScreen,
}

pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("[CLIENT] Initializing...");
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 900.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.5, 0.5, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    println!("[CLIENT] Initialized!");
}

pub fn update(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
    mut mouse: EventReader<bevy::input::mouse::MouseMotion>,
    mut camera: Query<&mut Transform, With<Camera3d>>,
) {
    if keyboard.pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
    let delta = time.delta_secs();
    // Note to self: Camera code will require a large rework when a proper player is added.
    let Ok(mut camera) = camera.single_mut() else {
        return;
    };
    const CAMERA_SPEED: f32 = 3.0;
    const CAMERA_ROTATION_SPEED: f32 = 0.03;
    for event in mouse.read() {
        camera.rotate_y(event.delta.x * delta * CAMERA_ROTATION_SPEED);
        camera.rotate_x(event.delta.y * delta * CAMERA_ROTATION_SPEED);
    }
    let mut offset = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) { offset += Vec3::new(-camera.rotation.z, 0.0, camera.rotation.x); }
    if keyboard.pressed(KeyCode::KeyA) { offset += Vec3::new(camera.rotation.x, 0.0, camera.rotation.z); }
    if keyboard.pressed(KeyCode::KeyS) { offset -= Vec3::new(-camera.rotation.z, 0.0, camera.rotation.x); }
    if keyboard.pressed(KeyCode::KeyD) { offset -= Vec3::new(camera.rotation.x, 0.0, camera.rotation.z); }
    if keyboard.pressed(KeyCode::Space) { offset += Vec3::Y; }
    if keyboard.pressed(KeyCode::ShiftLeft) { offset -= Vec3::Y; }
    camera.translation += offset.normalize_or_zero() * delta * CAMERA_SPEED;
}