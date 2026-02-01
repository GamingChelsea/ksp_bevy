use bevy::prelude::*;
use bevy_egui::*;
use big_space::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<TransformPlugin>())
        .add_plugins(EguiPlugin::default())
        .add_plugins(BigSpaceDefaultPlugins)
        .insert_resource(UniversalTime(0.0))
        .add_systems(Startup, setup)
        .add_systems(Update, (tick_universal_time, update_orbit))
        .run();
}

#[derive(Resource)]
struct UniversalTime(f64);

#[derive(Component)]
struct OrbitConfig {
    radius: f64,
    completion_speed: f64,
    start_phase: f64,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Erstelle das Universum (BigSpace + Grid auf einer Entity)
    commands.spawn_big_space(Grid::default(), |parent| {
        // 1. Die Kamera (Floating Origin)
        parent.spawn((
            Camera3d::default(),
            CellCoord::default(),
            FloatingOrigin,
            Transform::from_xyz(0.0, 20.0, 50.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
        ));

        // 2. Die Sonne (Zentrum)
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(5.0).mesh().ico(5).unwrap())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::from(bevy::color::palettes::basic::YELLOW),
                emissive: LinearRgba::rgb(1.0, 1.0, 0.5),
                ..default()
            })),
            CellCoord::default(),
        ));

        // 3. Ein Planet (Der sich bewegen wird)
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(2.0).mesh().ico(4).unwrap())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::from(bevy::color::palettes::basic::BLUE),
                ..default()
            })),
            CellCoord::default(),
            OrbitConfig {
                radius: 30.0,
                completion_speed: 0.5,
                start_phase: 0.0,
            },
        ));

        // 4. Ein weiterer Planet (Der sich bewegen wird)
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(2.0).mesh().ico(4).unwrap())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::from(bevy::color::palettes::basic::GREEN),
                ..default()
            })),
            CellCoord::default(),
            OrbitConfig {
                radius: 30.0,
                completion_speed: 0.5,
                start_phase: 1.0,
            },
        ));

        // Licht
        parent.spawn((
            DirectionalLight::default(),
            Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
            CellCoord::default(),
        ));
    });
}

fn tick_universal_time(time: Res<Time>, mut ut: ResMut<UniversalTime>) {
    ut.0 += time.delta_secs_f64();
}

fn update_orbit(
    ut: Res<UniversalTime>,
    grid: Single<&Grid>,
    mut query: Query<(&mut Transform, &mut CellCoord, &OrbitConfig)>,
) {
    for (mut transform, mut cell, orbit_config) in query.iter_mut() {
        let phase =
            (ut.0 * orbit_config.completion_speed) + orbit_config.start_phase;

        let x_abs = orbit_config.radius * phase.cos();
        let z_abs = orbit_config.radius * phase.sin();
        let y_abs = 0.0;

        // Nutze das Grid direkt (world_to_grid ist eine Methode von Grid)
        let (new_cell, new_translation) =
            grid.translation_to_grid([x_abs, y_abs, z_abs]);

        // Werte zuweisen
        *cell = new_cell;
        transform.translation = new_translation;
    }
}
