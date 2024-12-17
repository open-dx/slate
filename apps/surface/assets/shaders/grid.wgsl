@vertex
fn vertex_main(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
    return vec4(position, 0.0, 1.0);
}

@fragment
fn fragment_main(
    @builtin(position) frag_coord: vec4<f32>,
    @uniform camera_pos: vec2<f32>,
    @uniform grid_size: f32,
) -> @location(0) vec4<f32> {
    let world_pos = frag_coord.xy + camera_pos;

    // Calculate grid lines
    let line_thickness = 1.0;
    let grid_x = abs(fract(world_pos.x / grid_size - 0.5) - 0.5) < line_thickness / grid_size;
    let grid_y = abs(fract(world_pos.y / grid_size - 0.5) - 0.5) < line_thickness / grid_size;

    // Combine lines and set color
    let grid = grid_x || grid_y;
    let grid_color = vec3<f32>(0.3, 0.3, 0.3);

    return if grid {
        vec4(grid_color, 1.0)
    } else {
        vec4(0.0, 0.0, 0.0, 0.0)
    };
}
