#version 330

layout(location=0) in vec2 in_pos;
out vec3 out_col_vert;

void main() {
    gl_Position = vec4(in_pos, 0.0, 1.0);
    out_col_vert = vec3(1.0, 0.5, 0.0);
}
