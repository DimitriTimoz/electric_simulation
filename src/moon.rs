use bevy::prelude::*;

/// Component to mark the Moon entity
#[derive(Component)]
struct Moon;

pub struct MoonPlugin;

impl Plugin for MoonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_moon_system);
    }
}

fn spawn_moon_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create a sphere mesh for the moon
    let moon_mesh = meshes.add(Mesh::from(Sphere { radius: 30.0 }));

    // Create a slightly emissive material to make the moon glow
    let moon_material = StandardMaterial {
        base_color: Color::srgb(0.9, 0.9, 0.95),
        emissive: LinearRgba::rgb(0.3, 0.3, 0.4),
        unlit: false,
        ..default()
    };

    // Spawn the moon high up in the sky
    commands
        .spawn((
            Moon,
            Mesh3d::from(moon_mesh),
            Transform::from_translation(Vec3::new(100.0, 10000.0, 3000.0)),
            GlobalTransform::default(),
            MeshMaterial3d(materials.add(moon_material)),
        ))
        .insert(Name::new("Moon"));
}
