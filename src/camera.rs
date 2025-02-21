use bevy::{
    app::{App, Startup},
    core_pipeline::core_2d::Camera2d,
    ecs::{component::Component, system::Commands},
    math::Vec2,
    render::camera::OrthographicProjection,
};

#[derive(Component, Debug)]
#[require(Camera2d)]
pub struct OrthoCamera2d;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, initialize_2d_camera);
}

fn initialize_2d_camera(mut commands: Commands) {
    commands.spawn((
        OrthoCamera2d,
        Camera2d,
        OrthographicProjection {
            viewport_origin: Vec2::new(0.0, 1.0),
            ..OrthographicProjection::default_2d()
        },
    ));
}
