mod indexable;
mod inspector;

use core::f64;
use std::{
    cmp::{max, min},
    hash::Hash,
    ops::{Add, Deref, Div, Mul, Sub},
    sync::{Arc, RwLock},
    time::Duration,
};

use bevy::{
    color::palettes::css::PURPLE,
    math::ops::exp,
    pbr::MaterialPipelineKey,
    prelude::*,
    render::{
        camera::{CameraProjection, ScalingMode},
        mesh::{MeshVertexAttribute, MeshVertexBufferLayoutRef},
        render_resource::{
            encase::rts_array::Length, AsBindGroup, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError, VertexFormat,
        },
    },
    sprite::{Material2d, Material2dKey, Material2dPlugin},
    utils::{HashMap, HashSet},
    window::{PresentMode, WindowResized, WindowTheme},
};

use bevy_egui::EguiPlugin;
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_mod_index::{
    index::{Index, IndexInfo},
    prelude::IndexRefreshPolicy,
    storage::HashmapStorage,
};
use inspector::inspector_system;
use rand::Rng;

const SHADER_ASSET_PATH: &str = "shaders/shader.wgsl";

const SCANLINE_X: i32 = 17;
const SCANLINE_Y: i32 = 11;
pub const PACKED_SIZE: i32 = PixelPosition::pack(SCANLINE_X, SCANLINE_Y);
const STARTING_USER_PIXEL: PixelPosition = PixelPosition::new(SCANLINE_X / 2, SCANLINE_Y / 2);
const USER_PIXEL_OUTLINE_THICKNESS: f32 = 2.0;
const PIXEL_GAP: f32 = 5.0;
const PIXEL_SIZE: f32 = 56.0;
pub const PIXEL_WAIT_TIME: f64 = 50.0;

const ATTRIBUTE_PIXEL_SCALE: MeshVertexAttribute =
    MeshVertexAttribute::new("PixelSize", 94583659670978, VertexFormat::Float32x2);

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PixelMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[uniform(1)]
    brightness: f32,
    #[uniform(2)]
    outline_thickness: f32,
}

impl Material2d for PixelMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
            ATTRIBUTE_PIXEL_SCALE.at_shader_location(2),
        ])?;

        descriptor.vertex.buffers = vec![vertex_layout];

        Ok(())
    }
}

#[derive(Component, Default)]
pub struct PixelLifetime(f64);

impl PixelLifetime {
    // #[inline]
    // fn set(&mut self, time: Duration) {
    //     self.0 = time.as_millis() as u32;
    // }

    // #[inline]
    // fn to_brightness(&self, time: Duration) -> u32 {
    //     let current_time = time.as_millis() as u32;

    //     let t = 1 - ((current_time - self.0) / 6).clamp(0, 1);

    //     t.pow(2)
    // }
}

#[derive(Component, Default)]
pub struct PixelColor(u32);

#[derive(Component, Reflect, Default, Copy, Clone, Debug, Eq)]
pub struct PixelPosition {
    pos: IVec2,
    packed: i32,
}

impl PixelPosition {
    const fn new(x: i32, y: i32) -> Self {
        Self {
            pos: IVec2::new(x, y),
            packed: Self::pack(x, y),
        }
    }

    fn random() -> Self {
        let x = rand::rng().random_range(0..SCANLINE_X);
        let y = rand::rng().random_range(0..SCANLINE_Y);

        Self::new(x, y)
    }

    const fn pack(x: i32, y: i32) -> i32 {
        (y * SCANLINE_X) + x
    }

    fn unpack(pos: i32) -> IVec2 {
        IVec2::new(pos / SCANLINE_X, pos % SCANLINE_Y)
    }

    fn replace(&mut self, new_x: i32, new_y: i32) {
        self.pos = IVec2::new(new_x, new_y);
        self.packed = Self::pack(new_x, new_y);
    }

    fn normalised(&self) -> f64 {
        self.packed as f64 / PACKED_SIZE as f64
    }
}

impl Deref for PixelPosition {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.packed
    }
}

impl From<IVec2> for PixelPosition {
    fn from(value: IVec2) -> Self {
        Self {
            pos: value,
            packed: Self::pack(value.x, value.y),
        }
    }
}

impl From<(i32, i32)> for PixelPosition {
    fn from(value: (i32, i32)) -> Self {
        Self {
            pos: value.into(),
            packed: Self::pack(value.0, value.1),
        }
    }
}

impl From<i32> for PixelPosition {
    fn from(value: i32) -> Self {
        Self {
            pos: Self::unpack(value),
            packed: value,
        }
    }
}

impl PartialEq for PixelPosition {
    fn eq(&self, other: &PixelPosition) -> bool {
        self.packed == other.packed
    }
}

// impl PartialEq<(i32, i32)> for PixelPosition {
//     fn eq(&self, other: &(i32, i32)) -> bool {
//         self.packed == Self::pack(other.0, other.1)
//     }
// }

impl PartialEq<i32> for PixelPosition {
    fn eq(&self, other: &i32) -> bool {
        self.packed == *other
    }
}

// impl PartialOrd<(i32, i32)> for PixelPosition {
//     fn partial_cmp(&self, other: &(i32, i32)) -> Option<std::cmp::Ordering> {
//         PixelPosition::from(*other).partial_cmp(self)
//     }
// }

impl PartialOrd for PixelPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.packed.partial_cmp(&other.packed)
    }
}

impl PartialOrd<i32> for PixelPosition {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.packed.partial_cmp(other)
    }
}

impl Ord for PixelPosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.packed.cmp(&other.packed)
    }
}

impl Hash for PixelPosition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.packed.hash(state);
    }
}

#[derive(Reflect, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum NearestUserPixel {
    Ahead(PixelPosition),
    Wrap(PixelPosition),
}

impl NearestUserPixel {
    pub fn pos(&self) -> &PixelPosition {
        match self {
            NearestUserPixel::Ahead(pos) | NearestUserPixel::Wrap(pos) => pos,
        }
    }
}

impl PartialEq<NearestUserPixel> for PixelPosition {
    fn eq(&self, other: &NearestUserPixel) -> bool {
        match other {
            NearestUserPixel::Ahead(pos) | NearestUserPixel::Wrap(pos) => self.eq(pos),
        }
    }
}

impl PartialOrd<NearestUserPixel> for PixelPosition {
    fn partial_cmp(&self, other: &NearestUserPixel) -> Option<std::cmp::Ordering> {
        match other {
            NearestUserPixel::Ahead(pos) | NearestUserPixel::Wrap(pos) => self.partial_cmp(pos),
        }
    }
}

#[derive(Reflect, Resource)]
pub struct PixelStates {
    pub next_lit_pixel: PixelPosition,
    pub next_lit_time: f64,
    // pub nearest_user_pixel: NearestUserPixel,
}

impl PixelStates {
    #[inline]
    fn update_next_pixel(&mut self) {
        let next_x = (self.next_lit_pixel.pos.x + 1) % SCANLINE_X;
        let next_y = (self.next_lit_pixel.pos.y + (if next_x == 0 { 1 } else { 0 })) % SCANLINE_Y;

        self.next_lit_pixel.replace(next_x, next_y);
    }

    // fn update_next_lit_time(&mut self, elapsed: u64) {
    //     self.next_lit_time = elapsed + 100;
    // }
}

#[derive(Component, Default)]
#[require(Transform, Mesh2d, PixelColor)]
pub struct Pixel;

#[derive(Component, Reflect, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
pub struct PixelMarker {
    pos: PixelPosition,
}

#[derive(Component, Reflect, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
pub struct UserPixelMarker {
    pos: PixelPosition,
}

impl PartialEq<IVec2> for PixelMarker {
    fn eq(&self, other: &IVec2) -> bool {
        self.pos.pos == *other
    }
}

impl PixelMarker {
    pub fn get_translation(&self, window: &Window) -> Vec3 {
        let wres = &window.resolution;

        let pos_x = ((self.pos.pos.x as f32 - SCANLINE_X as f32 / 2.0) * (PIXEL_SIZE + PIXEL_GAP))
            + (wres.width() / 2.0);

        let pos_y = -((self.pos.pos.y as f32 - SCANLINE_Y as f32 / 2.0) * (PIXEL_SIZE + PIXEL_GAP))
            - (wres.height() / 2.0);

        Vec3::new(pos_x, pos_y, 1.0)
    }
}

type BellEasingFn = dyn Fn(f64) -> f64 + Send + Sync;

#[derive(Resource)]
pub struct CombinedEasing(Box<BellEasingFn>);

impl Default for CombinedEasing {
    fn default() -> Self {
        Self(Box::new(|x: f64| {
            bell_curve(x, STARTING_USER_PIXEL.normalised(), 0.2, 1.0)
        }))
    }
}

impl Deref for CombinedEasing {
    type Target = BellEasingFn;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Component)]
#[require(Pixel, PixelLifetime, MeshMaterial2d<PixelMaterial>, PixelMarker)]
pub struct GridPixel;

fn setup_grid(
    mut commands: Commands,
    window: Single<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PixelMaterial>>,
    // mut grid: ResMut<Grid>,
) {
    for x in 0..SCANLINE_X {
        for y in 0..SCANLINE_Y {
            let marker = PixelMarker {
                pos: PixelPosition::new(x, y),
            };

            let mut material = PixelMaterial {
                color: LinearRgba {
                    red: (x as f32 / SCANLINE_X as f32),
                    green: 1.0,
                    blue: (y as f32 / SCANLINE_Y as f32),
                    alpha: 1.0,
                },
                brightness: 0.0,
                outline_thickness: 0.0,
            };

            let mut pixel_entity = commands.spawn((
                Pixel,
                PixelColor(0),
                Transform::from_translation(marker.get_translation(&window)),
                GridPixel,
                PixelLifetime(0.0),
                marker,
            ));

            // grid.insert_pixel(marker.pos, pixel_entity.id());

            // TODO: make index
            if STARTING_USER_PIXEL.pos == (x, y).into() {
                material.outline_thickness = USER_PIXEL_OUTLINE_THICKNESS;

                pixel_entity.insert(UserPixelMarker {
                    pos: PixelPosition::new(x, y),
                });
                // grid.set_user_pixel(marker.pos);
            }

            let mesh = Mesh::from(Rectangle::new(PIXEL_SIZE, PIXEL_SIZE))
                .with_inserted_attribute(ATTRIBUTE_PIXEL_SCALE, vec![[PIXEL_SIZE, PIXEL_SIZE]; 4]);

            pixel_entity.insert((
                MeshMaterial2d(materials.add(material)),
                Mesh2d(meshes.add(mesh)),
            ));
        }
    }
}

fn scale<T>(number: T, in_min: T, in_max: T, out_min: T, out_max: T) -> T
where
    T: Sub<Output = T> + Div<Output = T> + Add<Output = T> + Mul<Output = T> + Copy,
{
    ((number - in_min) / (in_max - in_min)) * ((out_max - out_min) + out_min)
}

// fn bell_curve_easing(current_pixel: PixelPosition, nearest_user_pixel: PixelPosition) -> f64 {
//     let t = 50.0
//         * ((-((current_pixel.packed - nearest_user_pixel.packed) as f64 / 18.0).powf(2.0)) / 2.0)
//             .exp();
//     t
//     // scale(t, 1.1, 50.0, 0.005, 0.12)
// }

#[inline]
pub fn bell_curve(x: f64, center: f64, width: f64, sharpness: f64) -> f64 {
    (-((x - center).abs() / width).powf(sharpness)).exp()
}

// fn user_pixel_added(
//     mut materials: ResMut<Assets<PixelMaterial>>,
//     query: Query<(&UserPixelMarker, &MeshMaterial2d<PixelMaterial>), Added<UserPixelMarker>>,
// ) {
//     // dbg!(query.iter().size_hint());
//     if let Ok((_, mat_handle)) = query.get_single() {
//         materials
//             .get_mut(&mat_handle.0)
//             .expect("missing pixel material")
//             .outline_thickness = USER_PIXEL_OUTLINE_THICKNESS;
//     }

//     // for (marker) in &query2 {
//     //     dbg!("HI", marker);
//     // }
// }

fn user_pixel_added_observer(
    trigger: Trigger<OnAdd, UserPixelMarker>,
    query: Query<(&UserPixelMarker, &MeshMaterial2d<PixelMaterial>)>,
    mut materials: ResMut<Assets<PixelMaterial>>,
    mut easing_res: ResMut<CombinedEasing>,
) {
    if let Ok((_, mat_handle)) = query.get(trigger.entity()) {
        if let Some(mat) = materials.get_mut(&mat_handle.0) {
            mat.outline_thickness = USER_PIXEL_OUTLINE_THICKNESS;
        }
        // materials
        //     .get_mut(&mat_handle.0)
        //     .expect("missing pixel material")
        //     .outline_thickness = USER_PIXEL_OUTLINE_THICKNESS;
    }

    let mut max_ease: Box<BellEasingFn> = CombinedEasing::default().0;

    for (marker, ..) in &query {
        // done to keep the inner closure 'static
        let pixel_bell_curve_gen = |center: f64| move |x: f64| bell_curve(x, center, 0.2, 1.0);
        let pixel_bell_curve = pixel_bell_curve_gen(marker.pos.normalised());

        max_ease = Box::new(move |x| max_ease(x).max(pixel_bell_curve(x)));
    }

    easing_res.0 = max_ease;
}

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut game_state: ResMut<PixelStates>,
    mut index: Index<PixelMarker>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let new_pos = PixelPosition::random();

        if let Ok(entity) = index.lookup_single(&new_pos) {
            commands
                .entity(entity)
                .insert(UserPixelMarker { pos: new_pos });

            // // if we pick a pixel that is nearer to 0,0 than the existing user pixel, and it is
            // // not already behind the lit pixel, we update it
            // if new_pos < game_state.nearest_user_pixel && game_state.next_lit_pixel < new_pos {
            //     game_state.nearest_user_pixel = NearestUserPixel::Ahead(new_pos);
            // }
        }
    }
}

// fn update_nearest_user_pixel(
//     query: Query<&PixelMarker, With<UserPixelMarker>>,
//     mut game_state: ResMut<PixelStates>,
// ) {
//     // TODO: make `run_if`
//     let has_passed_user_pixel = game_state.next_lit_pixel >= game_state.nearest_user_pixel;
//     if has_passed_user_pixel && !matches!(game_state.nearest_user_pixel, NearestUserPixel::Wrap(..))
//     {
//         game_state.nearest_user_pixel = query
//             .iter()
//             .sort::<&PixelMarker>()
//             .find(|marker| marker.pos > game_state.next_lit_pixel)
//             .map_or_else(
//                 || NearestUserPixel::Wrap(query.iter().sort::<&PixelMarker>().next().unwrap().pos),
//                 |pos| NearestUserPixel::Ahead(pos.pos),
//             );
//     } else if !has_passed_user_pixel
//         && matches!(game_state.nearest_user_pixel, NearestUserPixel::Wrap(..))
//     {
//         // if we have wrapped back around to the beginning, make it `Ahead` again
//         game_state.nearest_user_pixel =
//             NearestUserPixel::Ahead(*game_state.nearest_user_pixel.pos())
//     }
// }

// fn ease_out_quad(x: f64) -> f64 {
//     1.0 - (1.0 - x) * (1.0 - x) // Quadratic easing out
// }
// fn ease_out_circ(x: f64) -> f64 {
//     (1.0 - (x - 1.0).powi(2)).sqrt()
// }
// fn ease_out_exponential(x: f64) -> f64 {
//     if x == 1.0 {
//         1.0
//     } else {
//         1.0 - 2.0f64.powf(-10.0 * x) // Strong falloff with a longer tail
//     }
// }

fn update_pixel_brightness(
    time: Res<Time>,
    mut materials: ResMut<Assets<PixelMaterial>>,
    query: Query<(&PixelMarker, &PixelLifetime, &MeshMaterial2d<PixelMaterial>)>,
) {
    let millis_elapsed = time.elapsed().as_millis() as f64;

    for (_, lifetime, mat_handle) in &query {
        let brightness =
            (1.0 - ((millis_elapsed - lifetime.0) / 1000.0 / 6.0).clamp(0.0, 1.0)).powf(3.0);

        if let Some(mat) = materials.get_mut(&mat_handle.0) {
            mat.brightness = brightness as f32;
        }
    }
}

fn update_pixel_lit_time(
    time: Res<Time>,
    mut state: ResMut<PixelStates>,
    mut query: Query<(&PixelMarker, &mut PixelLifetime)>,
    combined_easing: Res<CombinedEasing>,
    mut index: Index<PixelMarker>,
) {
    let millis_elapsed = time.elapsed().as_millis() as f64;

    // let trail_length = 30;

    // TODO: make `run_if`
    if millis_elapsed > state.next_lit_time {
        // these should always exist
        let pixel_entity = index
            .lookup_single(&state.next_lit_pixel)
            .expect("no entity for next_lit_pixel");

        let (_, mut lit_pixel_lifetime) = query
            .get_mut(pixel_entity)
            .expect("no component for next_lit_pixel entity");

        lit_pixel_lifetime.0 = millis_elapsed;

        state.update_next_pixel();

        state.next_lit_time =
            millis_elapsed + (PIXEL_WAIT_TIME * combined_easing(state.next_lit_pixel.normalised()));

        // for (pixel, mut lifetime) in &mut query {
        //     let distance = pixel.pos.abs_diff(state.next_lit_pixel.packed);

        //     if distance < trail_length {
        //         let fade_factor = distance as f64 / trail_length as f64;
        //         lifetime.0 = ease_out_exponential(1.0 - fade_factor); // Apply easing
        //     } else {
        //         lifetime.0 = 0.0; // Pixels beyond the trail are dark
        //     }

        //     // if pixel.pos == state.next_lit_pixel {
        //     //     lifetime.0 = 1.0;
        //     // } else {
        //     //     // lifetime.0 = 0.0;
        //     //     lifetime.0 = 0.0; //exponential_in(millis_elapsed - lifetime.0);
        //     // }
        // }

        // let eased_wait_time = bell_curve_easing(
        //     state.next_lit_pixel,
        //     *state.nearest_user_pixel.pos(),
        // );

        // let x = PixelPosition::pack(SCANLINE_X, SCANLINE_Y) - state.next_lit_pixel.packed;
        // let x = combined_easing(state.next_lit_pixel.packed as f64);
    }
}

fn position_grid_pixels(
    window: Option<Single<&Window>>,
    mut query: Query<(&PixelMarker, &mut Transform)>,
) {
    // TODO: make `run_if`
    if let Some(window) = window {
        for (pixel, mut transform) in &mut query {
            transform.translation = pixel.get_translation(&window);
        }
    }
}

fn setup(
    mut commands: Commands,
    window: Single<&Window>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PixelMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera2d,
        // change projection so top left is origin
        OrthographicProjection {
            viewport_origin: Vec2::new(0.0, 1.0),
            // scaling_mode: ScalingMode::AutoMax {
            //     max_width: 1920.0,
            //     max_height: 1080.0,
            // },
            ..OrthographicProjection::default_2d()
        },
    ));

    commands.insert_resource(PixelStates {
        next_lit_pixel: PixelPosition::new(0, 0),
        next_lit_time: 0.0, // next_lit_time: Duration::new(0, 0),
                            // nearest_user_pixel: NearestUserPixel::Ahead(STARTING_USER_PIXEL.into()),
    });

    setup_grid(commands, window, meshes, materials);
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
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
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            Material2dPlugin::<PixelMaterial>::default(),
            // #[cfg(debug_assertions)]
            // {
            //     WorldInspectorPlugin::new()
            // },
            EguiPlugin,
            DefaultInspectorConfigPlugin,
        ))
        .add_systems(Startup, (setup,))
        // .add_systems(PreUpdate, (update_nearest_user_pixel,))
        .add_systems(
            Update,
            (
                // user_pixel_added,
                update_pixel_lit_time,
                update_pixel_brightness,
                keyboard_input,
            ),
        )
        .add_systems(Update, inspector_system())
        .add_observer(user_pixel_added_observer)
        .add_systems(PostUpdate, (position_grid_pixels,))
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .insert_resource(CombinedEasing::default())
        .register_type::<HashmapStorage<PixelMarker>>()
        // .register_type::<PixelMarker>()
        // .register_type::<PixelPosition>()
        // .register_type::<NearestUserPixel>()
        // .register_type::<PixelStates>()
        .run();
}
