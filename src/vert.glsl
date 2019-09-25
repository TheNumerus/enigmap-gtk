#version 330

layout(location=0) in vec2 in_pos;
layout(location=1) in vec2 offset;
layout(location=2) in vec3 color;
out vec3 out_col_vert;
uniform float aspect_ratio;

void main() {
    vec2 pos = in_pos * 0.3 + offset;
    gl_Position = vec4(pos * vec2(1.0, aspect_ratio), 0.0, 1.0);
    out_col_vert = color;
}
