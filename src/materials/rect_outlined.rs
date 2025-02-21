use bevy::prelude::*;
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
    sprite::{AlphaMode2d, Material2d, Material2dKey, Material2dPlugin},
    utils::{HashMap, HashSet},
    window::{PresentMode, WindowResized, WindowTheme},
};

use super::ATTRIBUTE_RECT_SIZE;

const SHADER_ASSET_PATH: &str = "shaders/rect_outlined.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct OutlinedRectMaterial {
    #[uniform(0)]
    pub rect_color: LinearRgba,
    #[uniform(1)]
    pub outline_color: LinearRgba,
    #[uniform(2)]
    pub outline_thickness: f32,
}

impl Material2d for OutlinedRectMaterial {
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }

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
            ATTRIBUTE_RECT_SIZE.at_shader_location(2),
        ])?;

        descriptor.vertex.buffers = vec![vertex_layout];

        Ok(())
    }
}
