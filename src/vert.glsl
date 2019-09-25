#version 330

layout(location=0) in vec2 in_pos;
layout(location=1) in vec2 offset;
layout(location=2) in vec3 color;
out vec3 out_col_vert;

void main() {
    gl_Position = vec4(in_pos * 0.3 + offset, 0.0, 1.0);
    out_col_vert = color;
}
