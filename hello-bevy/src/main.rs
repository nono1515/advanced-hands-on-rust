use bevy::prelude::*;

const LEFT: Vec2 = Vec2::new(-1., 0.);
const RIGHT: Vec2 = Vec2::new(1., 0.);
const UP: Vec2 = Vec2::new(0., 1.);
const DOWN: Vec2 = Vec2::new(0., -1.);

#[derive(Component)]
struct Dragon;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(Camera2d);
  let dragon_image = asset_server.load("dragon.png");
  commands
    .spawn(Sprite::from_image(dragon_image))
    .insert(Dragon);
}

fn movement(
  input: Res<ButtonInput<KeyCode>>,
  mut dragon_query: Query<&mut Transform, With<Dragon>>,
) {
  let delta = if input.pressed(KeyCode::KeyW) {
    UP
  } else if input.pressed(KeyCode::KeyS) {
    DOWN
  } else if input.pressed(KeyCode::KeyA) {
    LEFT
  } else if input.pressed(KeyCode::KeyD) {
    RIGHT
  } else {
    Vec2::ZERO
  };
  dragon_query.iter_mut().for_each(|mut transform| {
    transform.translation += delta.extend(0.0);
  });
}

fn main() {
  App::new()
    .add_systems(Startup, setup)
    .add_systems(Update, movement)
    .add_plugins(DefaultPlugins)
    .run();
}
