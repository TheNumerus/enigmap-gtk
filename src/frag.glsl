#version 330

in vec3 out_col_vert;
out vec4 FragColor;

void main() {
    FragColor = vec4(out_col_vert, 1.0);
}