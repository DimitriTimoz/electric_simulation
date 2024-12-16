use std::time::Duration;

use bevy::{color::palettes::css::*, prelude::*};
use rand::Rng;
use rand_distr::{Distribution, Normal};

#[derive(Component)]
pub struct Conductive;

fn draw_segment(
    lightning: &mut Lightning,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    from: Vec3,
    to: Vec3,
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

    let e = commands.spawn((
        Mesh3d(cylinder_mesh.clone()),
        Transform::from_translation(origin_pos).with_rotation(rotation),
        lightning.material.clone(),
    ));
    lightning.rods.push(e.id());
}

#[derive(Component)]
pub struct Lightning {
    remaining_iterations: u32,
    last_points: Vec<Vec3>,
    material: MeshMaterial3d<StandardMaterial>,
    handle_material: Handle<StandardMaterial>,
    timer: Timer,
    rods: Vec<Entity>,
}

#[derive(Component)]
pub struct Rod;

pub fn setup_lightning(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    let mut from = Vec3::Y * 1000.0;
    from += Vec3::X * rand::thread_rng().gen_range(-300.0..300.0);
    let scaled_white = LinearRgba::from(BLUE) * 20.;
    let scaled_orange = LinearRgba::from(PURPLE) * 10.;
    let emissive = LinearRgba {
        red: scaled_white.red + scaled_orange.red,
        green: scaled_white.green + scaled_orange.green,
        blue: scaled_white.blue + scaled_orange.blue,
        alpha: 1.0,
    };
    let material = materials.add(StandardMaterial {
        emissive,
        diffuse_transmission: 1.0,
        ..default()
    });

    commands.spawn(Lightning {
        last_points: vec![from],
        remaining_iterations: 70,
        material: MeshMaterial3d(material.clone()),
        handle_material: material.clone(),
        timer: Timer::from_seconds(0.001, TimerMode::Repeating),
        rods: vec![],
    });
}

pub fn animate_lightning(
    mut commands: Commands,
    query: Query<&Transform, With<Conductive>>,
    mut query_lightning: Query<(Entity, &mut Lightning)>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut lightning) in query_lightning.iter_mut() {
        lightning.timer.tick(time.delta());
        if !lightning.timer.finished() {
            continue;
        }
        lightning.timer.reset();
        // S'arrêter si on a déjà fait toutes les itérations
        for _ in 0..1 {
            if lightning.remaining_iterations == 0 {
                for rod in lightning.rods.clone() {
                    let mut e = commands.entity(rod);
                    e.try_despawn();
                }
                commands.entity(entity).despawn();
                break;
            }
            let material = materials.get_mut(&lightning.handle_material).unwrap();
            material.emissive *= 1.01;
            let mut new_points = vec![];
            let mut rng = rand::thread_rng();

            'main: for last_point in lightning.last_points.clone() {
                let closest_conductive = query
                    .iter()
                    .map(|transform| transform.translation)
                    .map(|pos| {
                        let dist = 1.0 / (pos - last_point).length().powi(2);
                        (dist, pos)
                    })
                    .fold((0.0, Vec3::ZERO), |(sd, sv), (d, v)| (sd + d, sv + d * v));
                let closest_conductive = closest_conductive.1 / closest_conductive.0;

                // Correctly compute the direction from last_point to closest_conductive
                let direction = closest_conductive - last_point;
                let remaining_distance = direction.length();
                let normal =
                    Normal::new(0.0, (remaining_distance / 500.0).clamp(0.01, 0.4)).unwrap();

                let direction = direction.normalize();

                let min_amount_of_points = if new_points.is_empty() { 1 } else { 0 };
                let amount_of_points = rng.gen_range(min_amount_of_points..=2);
                for _ in 0..amount_of_points {
                    let direction_noised = direction
                        + Vec3::new(
                            normal.sample(&mut rng),
                            normal.sample(&mut rng),
                            normal.sample(&mut rng),
                        );
                    let (d, finished) = if remaining_distance < 5.0 {
                        (remaining_distance, true)
                    } else {
                        (
                            rng.gen_range(20.0..=remaining_distance.max(21.0).min(50.0)),
                            false,
                        )
                    };

                    let next_point = last_point + direction_noised * d;

                    new_points.push(next_point);

                    draw_segment(
                        &mut lightning,
                        &mut commands,
                        &mut meshes,
                        last_point,
                        next_point,
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
            }
            lightning.last_points = new_points;
            lightning.remaining_iterations = lightning.remaining_iterations.saturating_sub(1);
            if lightning.remaining_iterations == 0 {
                lightning.timer.set_duration(Duration::from_millis(200));
                lightning.timer.reset();
                material.emissive *= 10.0;
            }
        }
    }
}
