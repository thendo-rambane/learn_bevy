use bevy::prelude::*;

fn spawn_basic_scene(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands.spawn(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    ..default()
  });
}

fn spawn_camera(mut commands: Commands) {
  commands.spawn(Camera3dBundle {
    transform: Transform::from_xyz(-2.0, 2.5, 5.0)
      .looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
  });
}

fn main() {
  App::new()
    .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_basic_scene)
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      window: WindowDescriptor {
        title: "Bevy Tower Defence".to_string(),
        resizable: false,
        ..Default::default()
      },
      ..Default::default()
    }))
    .run()
}
