#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

@group(1) @binding(2)
var<uniform> wash_color_factor: f32;

fn pixelate(uv: vec2<f32>, size: vec2<f32>) -> vec2<f32> {
    return floor(uv * size) / size;
}

fn wash_color(num: f32, factor: f32) -> f32
{
    let f = pow(10.0, factor);
    return round(num*f)/f;
}

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    let uv = coords_to_viewport_uv(position.xy, view.viewport);
    var texture_uv = uv;

    //texture_uv = pixelate(texture_uv, vec2(1920.0/3.5, 1080.0/3.5));

    var output_color = vec4<f32>(
        wash_color(textureSample(texture, our_sampler, texture_uv).r, wash_color_factor),
        wash_color(textureSample(texture, our_sampler, texture_uv).g, wash_color_factor),
        wash_color(textureSample(texture, our_sampler, texture_uv).b, wash_color_factor),
        1.0,
    );


    return output_color;
}
