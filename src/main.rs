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
use rand::Rng;
pub mod clouds;
pub mod controllers;
pub mod lightning;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
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

    let angle_range = (-135.0f32).to_radians()..-45.0f32.to_radians();
    let rng = &mut rand::thread_rng();
    for _ in 0..50 {
        let sphere_mesh: Handle<Mesh> = meshes.add(Mesh::from(Sphere { radius: 1.0 }));

        let angle = rng.gen_range(angle_range.clone());
        let pos = Vec3::new(
            angle.cos() * 900.0,
            angle.sin() * 900.0
                + (((-90.0 - angle).abs().powf(1.5) / 90.0f32.powf(1.5))
                    * rand::random::<f32>()
                    * 300.0),
            0.0,
        );
        commands.spawn((
            Conductive,
            Mesh3d::from(sphere_mesh),
            Transform::from_translation(pos),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.8, 0.8),
                unlit: true,
                ..Default::default()
            })),
        ));
    }

    commands.insert_resource(CameraResource { zoom: 500.0 });
}
