use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::lightning::{setup_lightning, Rod};

#[derive(Resource)]
pub struct CameraResource {
    pub zoom: f32,
}

pub fn move_camera(
    mut evr_scroll: EventReader<MouseWheel>,
    time: Res<Time>,
    mut zoom: ResMut<CameraResource>,
    mut query: Query<(&mut Transform, &Camera3d)>,
) {
    for ev in evr_scroll.read() {
        zoom.zoom -= ev.y * 10.0;
    }

    for (mut transform, _camera) in query.iter_mut() {
        // Rotate the camera around the center of the world to a distance of 10.0 units
        let angle = (time.elapsed_secs() * 0.5).rem_euclid(2.0 * std::f32::consts::PI);
        transform.translation = Vec3::new(zoom.zoom * angle.cos(), 45.0, zoom.zoom * angle.sin());
        transform.look_at(Vec3::Y * 20.0, Vec3::Y);
    }
}

pub fn keyboard_input_system(
    query: Query<Entity, With<Rod>>,
    mut commands: Commands,
    materials: ResMut<Assets<StandardMaterial>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

    if keys.just_pressed(KeyCode::Space) {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
        setup_lightning(commands, materials);
    }
}
