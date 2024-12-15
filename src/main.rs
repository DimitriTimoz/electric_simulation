use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    prelude::*,
    render::{
        camera::Exposure,
        view::{ColorGrading, ColorGradingGlobal},
    },
};
use controllers::{keyboard_input_system, move_camera, CameraResource};
use lightning::{animate_lightning, setup_lightning, Conductive};
pub mod controllers;
pub mod lightning;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, setup_lightning))
        .add_systems(
            Update,
            (animate_lightning, move_camera, keyboard_input_system),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add a camera
    commands.spawn((
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Transform::from_xyz(300.0, 150.8, 10.0).looking_at(Vec3::ZERO + Vec3::Y * 30.0, Vec3::Y),
        ColorGrading {
            global: ColorGradingGlobal {
                post_saturation: 1.2,
                ..default()
            },
            ..default()
        },
        Tonemapping::TonyMcMapface,
        Exposure { ev100: 6.0 },
        Bloom::default(),
    ));

    let sphere_mesh = meshes.add(Mesh::from(Sphere { radius: 1.0 }));

    commands.spawn((
        Conductive,
        Mesh3d::from(sphere_mesh),
        Transform::from_xyz(0.0, 0.0, 0.0),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.8, 0.8),
            unlit: true,
            ..Default::default()
        })),
    ));

    commands.insert_resource(CameraResource { zoom: 300.0 });
}
