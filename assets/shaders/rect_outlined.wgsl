#import bevy_sprite::mesh2d_functions::{get_world_from_local, mesh2d_position_local_to_clip}

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) rect_size: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) rect_size: vec2<f32>,
};

@group(2) @binding(0) var<uniform> rect_color: vec4<f32>;
@group(2) @binding(1) var<uniform> outline_color: vec4<f32>;
@group(2) @binding(2) var<uniform> outline_thickness: f32;

@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    let normalised_scale_x = outline_thickness / input.rect_size.x;
    let normalised_scale_y = outline_thickness / input.rect_size.y;

    let scale_x = 1.0 + (normalised_scale_x * 2.0);
    let scale_y = 1.0 + (normalised_scale_y * 2.0);

    output.clip_position = mesh2d_position_local_to_clip(
        get_world_from_local(input.instance_index),
        vec4<f32>(input.position.x * scale_x, input.position.y * scale_y, input.position.z, 1.0),
    );

    output.uv = input.uv;
    output.rect_size = input.rect_size;

    return output;
}

fn inside_box(point: vec2<f32>, bottom_left: vec2<f32>, top_right: vec2<f32>) -> bool {
    return point.x > bottom_left.x && point.x < top_right.x && point.y > bottom_left.y && point.y > top_right.y;
}


@fragment
fn fragment(
    vertex_output: VertexOutput,
) -> @location(0) vec4<f32> {
    let uv = vertex_output.uv;

    let pixel = vertex_output.rect_size;

    let full_outline = pixel.x + outline_thickness * 2;
    let single_outline = pixel.x + outline_thickness;

    if outline_thickness > 0.0 && uv.x < outline_thickness / full_outline || uv.x > single_outline / full_outline || uv.y < outline_thickness / full_outline || uv.y > single_outline / full_outline {
         return outline_color;
    }

    return rect_color;
}
