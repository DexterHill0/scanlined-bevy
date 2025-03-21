use bevy::prelude::*;
use bevy::{math::IVec2, reflect::Reflect};
use std::hash::Hash;

#[derive(Reflect, Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct GridPosition {
    pub width: i32,
    pub height: i32,
    pub packed: i32,
}

impl GridPosition {
    pub const fn new(width: i32, height: i32, x: i32, y: i32) -> Self {
        Self {
            width,
            height,
            packed: Self::pack(width, x, y),
        }
    }

    pub const fn unpacked(&self) -> IVec2 {
        Self::unpack(self.width, self.height, self.packed)
    }

    pub fn replace(&mut self, pos: i32) {
        self.packed = pos;
    }

    pub fn replace_coords(&mut self, new_x: i32, new_y: i32) {
        self.packed = Self::pack(self.width, new_x, new_y);
    }

    pub fn normalised(&self) -> f64 {
        self.packed as f64 / Self::pack(self.width, self.width, self.height) as f64
    }

    pub const fn pack(width: i32, x: i32, y: i32) -> i32 {
        (y * width) + x
    }

    pub const fn unpack(width: i32, height: i32, pos: i32) -> IVec2 {
        IVec2::new(pos / width, pos % height)
    }
}
