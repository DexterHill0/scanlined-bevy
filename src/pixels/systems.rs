use bevy::prelude::*;
use bevy_mod_index::index::Index;

use crate::{
    grid::position::GridPosition,
    materials::rect_outlined::OutlinedRectMaterial,
    scenes::story::{CombinedBellEasing, PixelStates},
};

use super::{
    components::{Pixel, PixelLifetime, UserPixelMarker},
    PIXEL_WAIT_TIME, SCANLINE_X, SCANLINE_Y, USER_PIXEL_OUTLINE_THICKNESS,
};

pub(super) fn update_pixel_brightness(
    time: Res<Time>,
    mut materials: ResMut<Assets<OutlinedRectMaterial>>,
    query: Query<
        (
            &Pixel,
            &PixelLifetime,
            &MeshMaterial2d<OutlinedRectMaterial>,
        ),
        // dont update brightness of user pixels
        Without<UserPixelMarker>,
    >,
) {
    let millis_elapsed = time.elapsed().as_millis() as f64;

    for (_, lifetime, mat_handle) in &query {
        let brightness =
            (1.0 - ((millis_elapsed - **lifetime) / 1000.0 / 6.0).clamp(0.0, 1.0)).powf(3.0);

        if let Some(mat) = materials.get_mut(&mat_handle.0) {
            mat.rect_color.alpha = brightness as f32;
        }
    }
}

pub(super) fn update_pixel_lit_time(
    time: Res<Time>,
    mut state: ResMut<PixelStates>,
    mut query: Query<(&Pixel, &mut PixelLifetime), Without<UserPixelMarker>>,
    bell_easing: Res<CombinedBellEasing>,
    mut index: Index<Pixel>,
) {
    let millis_elapsed = time.elapsed().as_millis() as f64;

    // these should always exist
    let pixel_entity = index
        .lookup_single(&state.next_lit_pixel)
        .expect("no entity for next_lit_pixel");

    let (_, mut lit_pixel_lifetime) = query
        .get_mut(pixel_entity)
        .expect("no component for next_lit_pixel entity");

    **lit_pixel_lifetime = millis_elapsed;

    state.update_next_pixel();

    let x = state.next_lit_pixel.normalised();

    state.next_lit_time = millis_elapsed + (PIXEL_WAIT_TIME * bell_easing.evaluate((x, 0.2, 1.0)));
}

pub fn update_pixel_lit_time_run_if(time: Res<Time>, state: Res<PixelStates>) -> bool {
    let millis_elapsed = time.elapsed().as_millis() as f64;

    millis_elapsed > state.next_lit_time
}

pub(super) fn position_pixels(window: Single<&Window>, mut query: Query<(&Pixel, &mut Transform)>) {
    for (pixel, mut transform) in &mut query {
        transform.translation = pixel.get_translation(&window);
    }
}

pub(super) fn user_pixel_added_observer(
    trigger: Trigger<OnAdd, UserPixelMarker>,
    query: Query<(&UserPixelMarker, &MeshMaterial2d<OutlinedRectMaterial>)>,
    mut materials: ResMut<Assets<OutlinedRectMaterial>>,
    mut bell_easing: ResMut<CombinedBellEasing>,
) {
    if let Ok((_, mat_handle)) = query.get(trigger.entity()) {
        if let Some(mat) = materials.get_mut(&mat_handle.0) {
            mat.outline_thickness = USER_PIXEL_OUTLINE_THICKNESS;
        }
    }

    // TODO: use new struct methods
    // let mut max_ease: Box<BellEasingFn> = CombinedEasing::default().0;

    // for (marker, ..) in &query {
    //     // done to keep the inner closure 'static
    //     let pixel_bell_curve_gen = |center: f64| move |x: f64| bell_easing.evaluate(x, 0.2, 1.0);
    //     let pixel_bell_curve = pixel_bell_curve_gen(marker.pos.normalised());

    //     max_ease = Box::new(move |x| max_ease(x).max(pixel_bell_curve(x)));
    // }

    // easing_res.0 = max_ease;
}
