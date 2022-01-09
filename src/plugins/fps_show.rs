use bevy::{
  core::FixedTimestep,
  diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
  prelude::*,
};


pub struct MPlugin;


fn text_section_default(text: String, asset_server: &Res<AssetServer>) -> TextSection {
  TextSection {
    value: text,
    style: TextStyle {
      font: asset_server.load("fonts/FiraSans-Bold.ttf"),
      font_size: 20.0,
      color: Color::WHITE,
    },
  }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
struct FpsText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  #[cfg(debug_assertions)]
  println!("fps展示系统初始化");

  commands.spawn_bundle(UiCameraBundle::default());

  // 你如果要画一个文字 你要有一个文字实例
  commands.spawn_bundle(TextBundle {
    style: Style {
      position_type: PositionType::Absolute,
      position: Rect {
        top: Val::Px(5.0),
        left: Val::Px(5.0),
        ..Default::default()
      },
      ..Default::default()
    },
    text: Text {
      sections: vec![
        TextSection {
          ..text_section_default("fps:".to_string(), &asset_server)
        },
        TextSection {
          ..text_section_default("".to_string(), &asset_server)
        },
      ],
      alignment: Default::default(),
    },
    ..Default::default()
  }).insert(FpsText);
}

fn fps_update(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text>) {

    // 解构出fps
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
      if let Some(average) = fps.average() {
        for mut text in query.iter_mut() {
          text.sections[1].value = format!("{:.1}", average);
        }
      }
    }

}


impl Plugin for MPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(FrameTimeDiagnosticsPlugin::default())
      .add_startup_system(setup)
      .add_system_set(
        SystemSet::new()
          .with_run_criteria(FixedTimestep::step(0.5))
          .with_system(fps_update)
      )
    ;
  }
}

// 首先你要创建实例,意味着你需要setup吧,意味着你需要系统吧
// 我们先看看别人怎么做的。先看看效果，我们先翻译他的注释 他添加了一个帧数诊断