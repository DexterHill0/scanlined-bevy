use bevy::prelude::*;
use bevy_mod_index::index::Index;

use crate::{
    grid::position::GridPosition,
    pixels::{
        components::{Pixel, UserPixelMarker},
        SCANLINE_X, SCANLINE_Y,
    },
    scenes::story::PixelStates,
    utils::misc::random_grid_position,
};

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut index: Index<Pixel>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let new_pos = random_grid_position(SCANLINE_X, SCANLINE_Y);

        if let Ok(entity) = index.lookup_single(&new_pos) {
            commands.entity(entity).insert(UserPixelMarker);
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (keyboard_input,));
}
