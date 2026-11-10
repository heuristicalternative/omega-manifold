struct HexTensor {
    coords: array<i32, 6>,
    flags: u32,
};

@group(0) @binding(0) var<storage, read> input_manifold: array<HexTensor>;
@group(0) @binding(1) var<storage, read_write> output_manifold: array<HexTensor>;

const HEX_ROT_60: array<i32, 6> = array<i32, 6>(1, 2, 3, 4, 5, 0);

@compute @workgroup_size(64)
fn evaluate_hex_tensor(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let idx = global_id.x;
    let current_tensor = input_manifold[idx];

    var transformed: HexTensor;
    transformed.flags = current_tensor.flags;

    for (var i = 0; i < 6; i++) {
        let src_idx = HEX_ROT_60[i];
        transformed.coords[i] = current_tensor.coords[src_idx];
    }

    var checksum: i32 = 0;
    for (var i = 0; i < 6; i++) {
        checksum += transformed.coords[i];
    }
    
    if (checksum == 0) {
        output_manifold[idx] = transformed;
    } else {
        output_manifold[idx] = current_tensor;
    }
}
