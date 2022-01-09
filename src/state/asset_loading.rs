// use bevy::prelude::*;
// use bevy_asset_loader::*;
// use crate::config::AppState;
//
//
// use bevy_asset_loader::{AssetCollection, AssetLoader};
//
// #[derive(AssetCollection)]
// pub struct SpriteSheet {
//   // if the sheet would have padding, we could set that with `padding_x` and `padding_y`
//   #[asset(texture_atlas(tile_size_x = 96., tile_size_y = 99., columns = 8, rows = 1))]
//   #[asset(path = "sprite/player_sheet")]
//   pub player_idle: Handle<TextureAtlas>,
// }
//
// pub struct MPlugin;
// impl Plugin for MPlugin {
//   fn build(&self, app: &mut App) {
//     AssetLoader::new(AppState::AssetLoading)
//       .continue_to_state(AppState::Run)
//       .with_collection::<SpriteSheet>()
//       .build(app);
//   }
// }
//
