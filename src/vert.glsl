#version 330

layout(location=0) in vec2 in_pos;
layout(location=1) in vec2 offset;
layout(location=2) in vec3 color;
out vec3 out_col_vert;
uniform float aspect_ratio;
uniform float size_x;
uniform float size_y;

void main() {
    float map_asp = size_x / size_y;
    // create coords
    vec2 pos = (in_pos + offset);
    // move to center and scale to < -1.0, 1.0 >
    pos.y = -pos.y;
    pos.x /= size_x * 0.5;
    pos.y /= size_y * 0.5;
    pos -= vec2(1.0, -1.0);
    // aspect ratio correction
    pos /= vec2(max(1.0, 1.0 / map_asp), max(1.0, map_asp));
    
    if (map_asp >= 1.0) {
        if (aspect_ratio >= map_asp) {
            pos /= vec2(aspect_ratio / map_asp, 1.0 / map_asp);
        } else {
            pos /= vec2(1.0, 1.0 / aspect_ratio);
        }
    } else {
        if (aspect_ratio >= map_asp) {
            pos /= vec2(aspect_ratio, 1.0);
        } else {
            pos /= vec2(map_asp, map_asp / aspect_ratio);
        }
    }
    gl_Position = vec4(pos, 0.0, 1.0);
    out_col_vert = color;
}
