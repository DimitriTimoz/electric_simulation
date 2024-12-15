use bevy::{color::palettes::css::*, prelude::*};
use rand::Rng;

#[derive(Component)]
pub struct Conductive;

fn draw_segment(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    from: Vec3,
    to: Vec3,
    material: MeshMaterial3d<StandardMaterial>,
) {
    // Calcul de la direction et de la longueur
    let direction = to - from;
    let length = direction.length();

    // Création du cylindre (orientation par défaut le long de l'axe Y)
    let cylinder = Cylinder {
        radius: 0.5,
        half_height: length / 2.0,
    };
    let cylinder_mesh = meshes.add(cylinder.mesh());

    // Position du cylindre au milieu du segment
    let origin_pos = (from + to) / 2.0;

    // Normalisation de la direction
    let dir_norm = direction.normalize();
    // L'axe du cylindre est l'axe Y, on crée donc une rotation alignant Y sur dir_norm
    let rotation = Quat::from_rotation_arc(Vec3::Y, dir_norm);

    commands.spawn((
        Mesh3d(cylinder_mesh.clone()),
        Transform::from_translation(origin_pos).with_rotation(rotation),
        material,
    ));
}

#[derive(Resource)]
pub struct LightningState {
    remaining_iterations: u32,
    last_points: Vec<Vec3>,
    material: MeshMaterial3d<StandardMaterial>,
}

pub fn setup_lightning(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    let from = Vec3::Y * 100.0;

    let scaled_white = LinearRgba::from(BLUE) * 20.;
    let scaled_orange = LinearRgba::from(PURPLE) * 4.;
    let emissive = LinearRgba {
        red: scaled_white.red + scaled_orange.red,
        green: scaled_white.green + scaled_orange.green,
        blue: scaled_white.blue + scaled_orange.blue,
        alpha: 1.0,
    };
    let material = MeshMaterial3d(materials.add(StandardMaterial {
        emissive,
        diffuse_transmission: 1.0,
        ..default()
    }));

    commands.insert_resource(LightningState {
        last_points: vec![from],
        remaining_iterations: 10,
        material,
    });
}

pub fn animate_lightning(
    mut commands: Commands,
    query: Query<(&Transform, &Conductive)>,
    mut meshes: ResMut<Assets<Mesh>>,
    lightning: Option<ResMut<LightningState>>,
) {
    let pos_of_conductives = query.iter().map(|(t, _)| t.translation).collect::<Vec<_>>();
    if let Some(mut lightning) = lightning {
        // S'arrêter si on a déjà fait toutes les itérations
        if lightning.remaining_iterations == 0 {
            return;
        }

        let mut new_points = vec![];
        let mut rng = rand::thread_rng();

        'main: for last_point in lightning.last_points.clone() {
            let closest_conductive = pos_of_conductives
                .iter()
                .min_by_key(|pos| {
                    let dist = (*pos - last_point).length();
                    dist as i32
                })
                .unwrap();

            let direction = *closest_conductive - last_point;
            let remaining_distance = direction.length();
            let direction = direction.normalize();

            let min_amount_of_points = if new_points.is_empty() { 1 } else { 0 };
            let amount_of_points = rng.gen_range(min_amount_of_points..=3);
            for _ in 0..amount_of_points {
                const NOISE: f32 = 0.2;
                let direction_noised = direction
                    + Vec3::new(
                        rng.gen_range(-NOISE..=NOISE),
                        rng.gen_range(-NOISE..=NOISE),
                        rng.gen_range(-NOISE..=NOISE),
                    );

                let (d, finished) = if remaining_distance < 10.0 {
                    (remaining_distance, true)
                } else {
                    (rng.gen_range(10.0..=remaining_distance.min(50.0)), false)
                };

                let next_point = last_point + direction_noised * d;

                new_points.push(next_point);

                // Appel à votre fonction draw_segment
                draw_segment(
                    &mut commands,
                    &mut meshes,
                    last_point,
                    next_point,
                    lightning.material.clone(),
                );

                if finished {
                    new_points.clear();
                    break 'main;
                }
            }
        }

        if new_points.is_empty() {
            // Remove the lightning
            lightning.remaining_iterations = 0;

            return;
        }
        lightning.last_points = new_points;
        lightning.remaining_iterations -= 1;
    }
}
