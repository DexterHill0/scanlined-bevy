use bevy::prelude::*;
use bevy::{
    ecs::component::Component, math::Vec3, reflect::Reflect, render::mesh::Mesh2d,
    transform::components::Transform,
};
use bevy_mod_index::index::IndexInfo;
use bevy_mod_index::prelude::IndexRefreshPolicy;
use bevy_mod_index::storage::HashmapStorage;
use bevy_window::Window;

use crate::grid::position::GridPosition;

use super::{PIXEL_GAP, PIXEL_SIZE};

#[derive(Component, Reflect, Default)]
#[require(Transform, Mesh2d, PixelLifetime, PixelColor)]
pub struct Pixel {
    pub pos: GridPosition,
}

impl Pixel {
    pub fn get_translation(&self, window: &Window) -> Vec3 {
        let wres = &window.resolution;

        let grid_coords = self.pos.unpacked();

        let pos_x = ((grid_coords.x as f32 - self.pos.width as f32 / 2.0)
            * (PIXEL_SIZE + PIXEL_GAP))
            + (wres.width() / 2.0);

        let pos_y = -((grid_coords.y as f32 - self.pos.height as f32 / 2.0)
            * (PIXEL_SIZE + PIXEL_GAP))
            - (wres.height() / 2.0);

        Vec3::new(pos_x, pos_y, 1.0)
    }
}

impl IndexInfo for Pixel {
    type Component = Pixel;
    type Value = GridPosition;

    type Storage = HashmapStorage<Self>;

    const REFRESH_POLICY: IndexRefreshPolicy = IndexRefreshPolicy::WhenRun;

    fn value(c: &Self::Component) -> Self::Value {
        c.pos
    }
}

#[derive(Component, Reflect, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
pub struct PixelMarker;

#[derive(Component, Reflect, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
pub struct UserPixelMarker;

#[derive(Component, Default, Deref, DerefMut)]
pub struct PixelLifetime(pub f64);

#[derive(Component, Default, Deref)]
pub struct PixelColor(pub u32);
