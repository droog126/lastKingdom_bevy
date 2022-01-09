use bevy::app::{App, Plugin};
use bevy_inspector_egui::{ WorldInspectorPlugin };



pub struct DebugGuiPlugin;

impl Plugin for DebugGuiPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugin(WorldInspectorPlugin::new());
  }
}