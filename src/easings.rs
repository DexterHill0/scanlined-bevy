use std::{marker::PhantomData, ops::Deref};

use bevy::{ecs::system::Resource, utils::all_tuples};
use egui::emath::Real;

use crate::scenes::story::BellEasingRet;

/// max that only requires `T: PartialOrd` instead of `T: Ord`
fn loose_max<T: PartialOrd>(a: T, b: T) -> T {
    return if a <= b { b } else { a };
}

#[derive(Resource)]
pub struct CombinedEasing<Args, Ret> {
    cb: Box<dyn (Fn(Args) -> Ret) + Send + Sync>,
}

impl<Args, Ret> CombinedEasing<Args, Ret>
where
    Args: Copy + 'static,
    Ret: PartialOrd + Copy + 'static,
{
    pub fn new<F>(easing_fn: F) -> Self
    where
        F: Fn(Args) -> Ret + Send + Sync + 'static,
    {
        Self {
            cb: Box::new(easing_fn),
        }
    }

    pub fn extend<F>(&mut self, next: F)
    where
        F: Fn(Args) -> Ret + Send + Sync + 'static,
    {
        let current = std::mem::replace(&mut self.cb, Box::new(|_| panic!("should not be called")));

        self.cb = Box::new(move |x: Args| loose_max(current(x), next(x)));
    }

    pub fn evaluate(&self, x: Args) -> Ret {
        (self.cb)(x)
    }
}

#[inline]
pub fn bell_curve((x, center, width, sharpness): (f64, f64, f64, f64)) -> BellEasingRet {
    (-((x - center).abs() / width).powf(sharpness)).exp()
}
