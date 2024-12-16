use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::{
        camera::Exposure,
        view::{ColorGrading, ColorGradingGlobal},
    },
};

use crate::lightning::{setup_lightning, LightningMaterial};
pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraController {
            speed: 500.0,
            sensitivity: 0.1,
            zoom_sensitivity: 0.5,
            pitch: 0.0,
            yaw: 0.0,
            min_zoom: 1.0,
            max_zoom: 5000.0,
            distance: 500.0,
        })
        .add_systems(Startup, setup_camera_system)
        .add_systems(
            Update,
            (
                camera_movement_system,
                camera_mouse_input_system,
                camera_zoom_system,
            ),
        );
    }
}

/// A resource to hold camera movement parameters.
#[derive(Resource)]
struct CameraController {
    pub speed: f32,
    pub sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub distance: f32,
}

/// Spawn a camera and a light source.
fn setup_camera_system(mut commands: Commands) {
    commands
        .spawn((
            Camera3d::default(),
            Camera {
                hdr: true,
                ..default()
            },
            Bloom::default(),
            Transform::from_xyz(0.0, 300.0, 1300.0).looking_at(Vec3::ZERO, Vec3::Y),
        ))
        .insert(Name::new("MainCamera"));
}

/// System to handle keyboard input for camera movement and vertical movement.
fn camera_movement_system(
    commands: Commands,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    materials: ResMut<Assets<LightningMaterial>>,
    controller: ResMut<CameraController>,
) {
    for mut transform in query.iter_mut() {
        // Calculate forward (front) direction ignoring pitch for WASD movement
        let forward = Vec3::new(transform.forward().x, 0.0, transform.forward().z).normalize();
        let right = transform.right();

        let mut velocity = Vec3::ZERO;
        // Forward/Back
        if keyboard.pressed(KeyCode::KeyW) {
            velocity += forward;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            velocity -= forward;
        }
        // Left/Right
        if keyboard.pressed(KeyCode::KeyA) {
            velocity -= right * 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            velocity += right * 1.0;
        }
        // Up/Down
        if keyboard.pressed(KeyCode::KeyQ) {
            velocity.y += 1.0;
        }

        if keyboard.pressed(KeyCode::KeyE) {
            velocity.y -= 1.0;
        }

        if keyboard.pressed(KeyCode::ShiftLeft) {
            velocity *= 2.0;
        }

        // Normalize diagonal movement
        if velocity.length_squared() > 1e-6 {
            velocity = velocity.normalize();
        }

        transform.translation += velocity * controller.speed * time.delta_secs();
        // Update rotation based on pitch/yaw stored in the controller
        let yaw_quat = Quat::from_rotation_y(controller.yaw);
        let pitch_quat = Quat::from_rotation_x(controller.pitch);
        transform.rotation = yaw_quat * pitch_quat;
    }

    if keyboard.just_released(KeyCode::Space) {
        setup_lightning(commands, materials);
    }

    if keyboard.just_released(KeyCode::Escape) {
        std::process::exit(0);
    }
}

/// System to handle mouse movement for camera rotation.
fn camera_mouse_input_system(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut motion_events: EventReader<MouseMotion>,
    mut controller: ResMut<CameraController>,
) {
    if mouse_buttons.pressed(MouseButton::Right) {
        for event in motion_events.read() {
            controller.yaw -= event.delta.x * controller.sensitivity * 0.01;
            controller.pitch -= event.delta.y * controller.sensitivity * 0.01;
            // Clamp pitch so we don't flip over
            controller.pitch = controller.pitch.clamp(-1.54, 1.54);
        }
    }
}

/// System to handle mouse wheel zoom.
fn camera_zoom_system(
    mut scroll_events: EventReader<MouseWheel>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    mut controller: ResMut<CameraController>,
) {
    for event in scroll_events.read() {
        controller.distance -= event.y * controller.zoom_sensitivity;
        controller.distance = controller
            .distance
            .clamp(controller.min_zoom, controller.max_zoom);

        // Adjust camera position along its forward axis
        if let Ok(mut transform) = query.get_single_mut() {
            // Move the camera forward/back based on distance
            let forward = transform.forward();
            let current_pos = transform.translation;
            let target_pos = current_pos + forward * event.y * controller.zoom_sensitivity;

            // Set the translation while keeping the rotation intact
            transform.translation = transform.translation.lerp(target_pos, 0.5);
        }
    }
}
