use bevy::prelude::*;
use config::AppState;

mod config;
mod plugins;
mod state;

use bevy_asset_loader::{AssetCollection, AssetLoader};

fn main(){
  let mut app = App::new();
  //config
  app.add_plugin(config::windows::Config);


  //plugin
  app.add_plugin(plugins::fps_show::MPlugin);
  #[cfg(debug_assertions)]
    app.add_plugin(plugins::debug_gui::DebugGuiPlugin);


  AssetLoader::new(AppState::AssetLoading)
    .continue_to_state(AppState::Run)
    // .with_collection::<SpriteSheet>()
    .build(&mut app);

  app
    .add_state(AppState::AssetLoading)
    .add_system_set(
      SystemSet::on_enter(AppState::Run)
        .with_system(state::run::enter)
    )
    .add_system_set(
      SystemSet::on_update(AppState::Run)
        .with_system(state::run::update)
    );


  app.run();
}

#[derive(AssetCollection)]
struct SpriteSheet {
  // if the sheet would have padding, we could set that with `padding_x` and `padding_y`
  #[asset(texture_atlas(tile_size_x = 96., tile_size_y = 99., columns = 8, rows = 1))]
  #[asset(path = "sprite/player_sheet.png")]
  female_adventurer: Handle<TextureAtlas>,
}