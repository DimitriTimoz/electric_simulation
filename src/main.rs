use bevy::prelude::*;
use lightning::{animate_lightning, setup_lightning, Conductive};
use rand::Rng;
pub mod clouds;
pub mod controllers;
pub mod lightning;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, controllers::CameraControllerPlugin))
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, (setup, setup_lightning))
        .add_systems(Update, animate_lightning)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
 

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
}
