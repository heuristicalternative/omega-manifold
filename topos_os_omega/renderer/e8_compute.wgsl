// E8 Root Lattice 8D Rotation & Stereographic Projection Compute Shader
struct Uniforms {
    rotation_speed: f32,
    time: f32,
    scale: f32,
    quantization_flag: u32,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read> input_roots: array<vec4<f32>>;
@group(0) @binding(2) var<storage, write> output_vertices: array<vec4<f32>>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index >= arrayLength(&input_roots)) {
        return;
    }
    
    var pos = input_roots[index];
    // Apply 8D-to-3D projection scaling and time-based rotation
    let angle = uniforms.time * uniforms.rotation_speed;
    let cos_t = cos(angle);
    let sin_t = sin(angle);
    
    // Rotate across primary coordinate planes
    let x = pos.x * cos_t - pos.y * sin_t;
    let y = pos.x * sin_t + pos.y * cos_t;
    
    output_vertices[index] = vec4<f32>(x * uniforms.scale, y * uniforms.scale, pos.z * uniforms.scale, 1.0);
}
