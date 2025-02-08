#import bevy_sprite::mesh2d_functions::{get_world_from_local, mesh2d_position_local_to_clip}

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) pixel_size: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) pixel_size: vec2<f32>,
};

@group(2) @binding(0) var<uniform> pixel_color: vec4<f32>;
@group(2) @binding(1) var<uniform> pixel_brightness: f32;
@group(2) @binding(2) var<uniform> outline_thickness: f32;

@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    let normalised_scale_x = outline_thickness / input.pixel_size.x;
    let normalised_scale_y = outline_thickness / input.pixel_size.y;

    let scale_x = 1.0 + (normalised_scale_x * 2.0);
    let scale_y = 1.0 + (normalised_scale_y * 2.0);

    output.clip_position = mesh2d_position_local_to_clip(
        get_world_from_local(input.instance_index),
        vec4<f32>(input.position.x * scale_x, input.position.y * scale_y, input.position.z, 1.0),
    );

    output.uv = input.uv;
    output.pixel_size = input.pixel_size;

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

    let pixel = vertex_output.pixel_size;

    let big = pixel.x + outline_thickness * 2;
    let gongag = pixel.x + outline_thickness;

    if outline_thickness > 0.0 && uv.x < outline_thickness / big || uv.x > gongag / big || uv.y < outline_thickness / big || uv.y > gongag / big {
         return vec4(1.0, 1.0, 1.0, 1.0);
    }

    return vec4(f32(pixel_brightness), 0.0, 0.0, 1.0);
}
