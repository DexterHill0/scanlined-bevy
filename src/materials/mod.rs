pub mod rect_outlined;

use bevy::{
    app::App,
    render::{mesh::MeshVertexAttribute, render_resource::VertexFormat},
    sprite::Material2dPlugin,
};
use rect_outlined::OutlinedRectMaterial;

pub const ATTRIBUTE_RECT_SIZE: MeshVertexAttribute =
    MeshVertexAttribute::new("RectSize", 94583659670978, VertexFormat::Float32x2);

pub fn plugin(app: &mut App) {
    app.add_plugins((Material2dPlugin::<OutlinedRectMaterial>::default(),));
}
