use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::{
    app::App,
    ecs::{
        query::With,
        schedule::{IntoSystemConfigs, NodeConfigs},
        system::{Res, ResMut, System},
        world::World,
    },
    input::{common_conditions::input_toggle_active, keyboard::KeyCode},
    time::Time,
};
use bevy_egui::{EguiContext, EguiPlugin};
use bevy_inspector_egui::{
    bevy_inspector::{self, hierarchy::SelectedEntities},
    DefaultInspectorConfigPlugin,
};
use bevy_mod_index::storage::HashmapStorage;
use bevy_window::PrimaryWindow;
use egui::TextEdit;
use egui_plot::{Line, Plot, PlotPoints, Points};

use crate::pixels::components::Pixel;
use crate::pixels::PIXEL_WAIT_TIME;
use crate::scenes::story::{CombinedBellEasing, PixelStates};

pub fn plugin(app: &mut App) {
    app.register_type::<HashmapStorage<Pixel>>();

    app.add_plugins((
        LogDiagnosticsPlugin::default(),
        FrameTimeDiagnosticsPlugin,
        EguiPlugin,
        DefaultInspectorConfigPlugin,
    ));
}

fn inspector_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::SidePanel::left("inspector")
        .default_width(250.0)
        .show(egui_context.get_mut(), |ui| {
            ui.heading("Inspector");

            egui::ScrollArea::both().show(ui, |ui| {
                egui::CollapsingHeader::new("Entities").show(ui, |ui| {
                    bevy_inspector::ui_for_entities(world, ui);
                });
                egui::CollapsingHeader::new("Resources").show(ui, |ui| {
                    bevy_inspector::ui_for_resources(world, ui);
                });
                egui::CollapsingHeader::new("Assets").show(ui, |ui| {
                    bevy_inspector::ui_for_all_assets(world, ui);
                });

                ui.allocate_space(ui.available_size());
            });
        });

    let bell_curve_easing = world.resource::<CombinedBellEasing>();
    let game_state = world.resource::<PixelStates>();

    egui::SidePanel::right("extras_inspector")
        .default_width(250.0)
        .show(egui_context.get_mut(), |ui| {
            ui.heading("Extras");

            egui::ScrollArea::both().show(ui, |ui| {
                {
                    let x = game_state.next_lit_pixel.normalised();

                    let current_easing_val = bell_curve_easing.evaluate((x, 0.2, 1.0));

                    {
                        ui.label("Pixel Easing Curve");

                        let sin: PlotPoints = (0..100)
                            .map(|i| {
                                let x = i as f64 / 100.0;

                                [x, bell_curve_easing.evaluate((x, 0.2, 1.0))]
                            })
                            .collect();

                        let current_position = Points::new([
                            game_state.next_lit_pixel.normalised(),
                            current_easing_val,
                        ])
                        .radius(6.0);

                        let line = Line::new(sin);
                        Plot::new("easing_plot")
                            .view_aspect(2.0)
                            .show(ui, |plot_ui| {
                                plot_ui.line(line);
                                plot_ui.points(current_position);
                            });
                    }
                    {
                        ui.label("Time between pixels (ms)");
                        let mut val = format!("{:.2}", PIXEL_WAIT_TIME * current_easing_val);
                        ui.text_edit_singleline(&mut val);
                    }
                }

                ui.allocate_space(ui.available_size());
            });
        });
}

pub fn inspector_system() -> NodeConfigs<Box<(dyn System<In = (), Out = ()> + 'static)>> {
    inspector_ui.run_if(input_toggle_active(false, KeyCode::Escape))
}
