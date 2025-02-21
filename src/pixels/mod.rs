pub mod components;
pub mod systems;

use bevy::{
    app::{App, Update},
    ecs::{
        schedule::{common_conditions::resource_exists, IntoSystemConfigs},
        system::Single,
    },
    state::condition::in_state,
};
use bevy_window::Window;
use systems::{
    position_pixels, update_pixel_brightness, update_pixel_lit_time, update_pixel_lit_time_run_if,
    user_pixel_added_observer,
};

use crate::{
    grid::position::GridPosition,
    scenes::{story::PixelStates, SceneState},
    utils::run_if::has_window,
};

pub const SCANLINE_X: i32 = 17;
pub const SCANLINE_Y: i32 = 11;
pub const PACKED_SIZE: i32 = GridPosition::pack(SCANLINE_X, SCANLINE_X, SCANLINE_Y);
pub const STARTING_USER_PIXEL: GridPosition =
    GridPosition::new(SCANLINE_X, SCANLINE_Y, SCANLINE_X / 2, SCANLINE_Y / 2);
pub const USER_PIXEL_OUTLINE_THICKNESS: f32 = 2.0;
pub const PIXEL_GAP: f32 = 5.0;
pub const PIXEL_SIZE: f32 = 56.0;
pub const PIXEL_WAIT_TIME: f64 = 50.0;

pub fn plugin(app: &mut App) {
    app.add_observer(user_pixel_added_observer);
    app.add_systems(
        Update,
        (
            update_pixel_brightness,
            update_pixel_lit_time.run_if(update_pixel_lit_time_run_if),
            position_pixels.run_if(has_window),
        )
            .run_if(in_state(SceneState::Game)),
    );
}
