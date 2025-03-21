use bevy::prelude::*;
use bevy_mod_index::index::Index;

use crate::{
    grid::position::GridPosition,
    pixels::{
        components::{AllPixels, Pixel, UserPixelMarker},
        systems::set_user_pixel,
        SCANLINE_X, SCANLINE_Y,
    },
    scenes::story::PixelStates,
    utils::misc::random_grid_position,
};

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    index: Index<AllPixels>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let new_pos = random_grid_position(SCANLINE_X, SCANLINE_Y);

        set_user_pixel(&mut commands, index, new_pos);
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (keyboard_input,));
}
