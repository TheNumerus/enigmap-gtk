#version 330

layout(location=0) in vec2 in_pos;
layout(location=1) in vec2 offset;
layout(location=2) in vec3 color;
out vec3 out_col_vert;
uniform float aspect_ratio;
uniform float size_x;
uniform float size_y;

void main() {
    vec2 pos = (in_pos + offset);// * vec2(1.0, aspect_ratio);
    pos.y = -pos.y;
    pos.x /= size_x * 0.5;
    pos.y /= size_y * 0.5;
    pos -= vec2(1.0, -1.0);
    gl_Position = vec4(pos, 0.0, 1.0);
    out_col_vert = color;
}
