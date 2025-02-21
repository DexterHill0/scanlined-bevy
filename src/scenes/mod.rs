pub mod story;

use bevy::prelude::*;
use story::setup_game_scene;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum SceneState {
    #[default]
    MainMenu,
    Game,
}

pub fn plugin(app: &mut App) {
    app.init_state::<SceneState>();
    app.enable_state_scoped_entities::<SceneState>();
    app.add_systems(OnEnter(SceneState::Game), setup_game_scene);
}
