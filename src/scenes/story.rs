use bevy::prelude::*;

use crate::{
    easings::{bell_curve, CombinedEasing},
    grid::position::GridPosition,
    materials::{rect_outlined::OutlinedRectMaterial, ATTRIBUTE_RECT_SIZE},
    pixels::{
        components::{Pixel, PixelColor, PixelLifetime, PixelMarker},
        PACKED_SIZE, PIXEL_SIZE, SCANLINE_X, SCANLINE_Y,
    },
};

use super::SceneState;

#[derive(Reflect, Resource, Default)]
pub struct PixelStates {
    pub next_lit_pixel: GridPosition,
    pub next_lit_time: f64,
}

impl PixelStates {
    #[inline]
    pub fn update_next_pixel(&mut self) {
        let next_pos = (self.next_lit_pixel.packed + 1) % PACKED_SIZE;

        self.next_lit_pixel.replace(next_pos);
    }
}

pub type BellEasingArgs = (f64, f64, f64);
pub type BellEasingRet = f64;

pub type CombinedBellEasing = CombinedEasing<BellEasingArgs, BellEasingRet>;

// TODO: use required component for scene entities?
// #[derive(Component)]
// pub struct StoryScene;

pub(super) fn setup_game_scene(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    outline_mats: ResMut<Assets<OutlinedRectMaterial>>,
) {
    commands.init_resource::<PixelStates>();

    setup_pixel_grid(commands, meshes, outline_mats);
}

// TODO: move this into an observer probably
// if STARTING_USER_PIXEL.pos == (x, y).into() {
//     material.outline_thickness = USER_PIXEL_OUTLINE_THICKNESS;

//     pixel_entity.insert(UserPixelMarker {
//         pos: GridPosition::new(x, y),
//     });
//     // grid.set_user_pixel(marker.pos);
// }

fn setup_pixel_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<OutlinedRectMaterial>>,
) {
    for x in 0..SCANLINE_X {
        for y in 0..SCANLINE_Y {
            let pixel_mesh = Mesh::from(Rectangle::new(PIXEL_SIZE, PIXEL_SIZE))
                .with_inserted_attribute(ATTRIBUTE_RECT_SIZE, vec![[PIXEL_SIZE, PIXEL_SIZE]; 4]);

            let outlined_mat = OutlinedRectMaterial {
                rect_color: LinearRgba {
                    red: (x as f32 / SCANLINE_X as f32),
                    green: 1.0,
                    blue: (y as f32 / SCANLINE_Y as f32),
                    alpha: 1.0,
                },
                outline_color: LinearRgba::WHITE,
                outline_thickness: 0.0,
            };

            commands.spawn((
                StateScoped(SceneState::Game),
                Pixel {
                    pos: GridPosition::new(SCANLINE_X, SCANLINE_Y, x, y),
                },
                PixelColor(0),
                PixelLifetime(0.0),
                MeshMaterial2d(materials.add(outlined_mat)),
                Mesh2d(meshes.add(pixel_mesh)),
            ));
        }
    }
}
