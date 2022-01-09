use bevy::prelude::*;

pub struct Config;

fn start(mut commands:Commands){
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

impl Plugin for Config {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(WindowDescriptor{
        title: "BevyMark".to_string(),
        width: 800.,
        height: 600.,
        vsync: false,
        resizable: true,
        ..Default::default()
      })
      .add_plugins(DefaultPlugins)
      .add_startup_system(start);
  }
}