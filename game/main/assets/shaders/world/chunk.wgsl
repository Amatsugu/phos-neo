#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}
#import bevy_pbr::mesh_functions::{mesh_position_local_to_world,get_model_matrix,mesh_normal_local_to_world}
#import bevy_pbr::view_transformations::position_world_to_clip;

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif


@group(2) @binding(100) var array_texture: texture_2d_array<f32>;
@group(2) @binding(101) var array_texture_sampler: sampler;
@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
//	var vin : VertexOutput;
//	vin.position = in.position;
//	vin.world_position = in.world_position;
//	vin.world_normal = in.world_normal;
//	vin.uv = in.uv;

    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);


    // alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    // apply lighting

	let index = floor(in.uv.x - 1) + 1;
    var uv = in.uv;
    uv.x = in.uv.x - index;
    out.color = textureSample(array_texture, array_texture_sampler, uv, u32(index));

    out.color *= apply_pbr_lighting(pbr_input);



    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

#endif

    return out;
}

//struct Vertex {
//    @builtin(instance_index) instance_index: u32,
//    @location(0) position: vec3<f32>,
//    @location(1) uv: vec2<f32>,
//    @location(2) normal: vec3<f32>,
//    @location(3) texture_index: u32,
//};
//
//struct VOut {
//    @builtin(position) position: vec4<f32>,
//	@location(0) world_position: vec4<f32>,
//	@location(1) world_normal: vec3<f32>,
//	@location(2) uv: vec2<f32>,
////    @location(7) @interpolate(flat) texture_index: u32,
//};
//
//@vertex
//fn vertex(vertex: Vertex) -> VOut {
//    var out: VOut;
//    out.world_position = mesh_position_local_to_world(get_model_matrix(vertex.instance_index), vec4<f32>(vertex.position, 1.0));
//    out.position = position_world_to_clip(out.world_position.xyz);
////    out.texture_index = vertex.texture_index;
//    out.uv = vertex.uv;
//    out.world_normal = mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
//    return out;
//}
