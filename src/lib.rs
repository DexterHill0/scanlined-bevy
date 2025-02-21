mod camera;
#[cfg(debug_assertions)]
mod debug;
mod easings;
mod grid;
mod input;
mod materials;
mod pixels;
mod scenes;
mod utils;
mod window;

use bevy::prelude::*;

use bevy::prelude::PluginGroup;
use bevy::{
    app::{App, Plugin},
    DefaultPlugins,
};
use bevy_window::{PresentMode, Window, WindowPlugin, WindowTheme};

pub struct ScanlinedApp;

impl Plugin for ScanlinedApp {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            window::plugin,
            camera::plugin,
            materials::plugin,
            pixels::plugin,
            scenes::plugin,
            input::plugin,
        ));

        #[cfg(debug_assertions)]
        app.add_plugins((debug::plugin,));
    }
}
