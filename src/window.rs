use bevy::prelude::*;
use bevy_window::{PresentMode, WindowTheme};

const BACKGROUND_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);

pub(super) fn plugin(app: &mut App) {
    let primary_window = Window {
        title: "Scanlined".into(),
        name: Some("scanlined.app".into()),
        resolution: (1200., 900.).into(),
        present_mode: PresentMode::AutoVsync,
        fit_canvas_to_parent: true,
        prevent_default_event_handling: false,
        window_theme: Some(WindowTheme::Dark),
        enabled_buttons: bevy::window::EnabledButtons {
            maximize: false,
            ..Default::default()
        },
        visible: true,
        ..default()
    };

    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(primary_window),
            ..default()
        }));
}
