#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#define GLEW_STATIC
#include <GL/glew.h>
#include <stdint.h>

// RUST FUNCTIONS
extern void get_hex_verts(float* verts);

// CONSTANTS
const uint32_t indices[] = {
    5, 4, 0, 3, 1, 2
};

// 6 verts with 2 coords each
const int32_t verts_len = 12; 

// GLOBALS
char * vert_shader_source = NULL;
char * frag_shader_source = NULL;
uint32_t vao, vbo, ebo, vbo_instances, ratio_uniform, uniform_size_x, uniform_size_y, shader_program, uniform_zoom;

float verts[12];

uint32_t instances_len;

// INTERNAL FUNCTIONS
void generate_program() {
    uint32_t vert_sh, frag_sh;
    int32_t success;

    vert_sh = glCreateShader(GL_VERTEX_SHADER);
    glShaderSource(vert_sh, 1, (const char* const*)&vert_shader_source, NULL);
    glCompileShader(vert_sh);

    char infoLog[512];
    glGetShaderiv(vert_sh, GL_COMPILE_STATUS, &success);
    if (!success) {
        glGetShaderInfoLog(vert_sh, 512, NULL, infoLog);
        printf("Error: vertex shader compilation failed: %s\n",infoLog);
    }

    frag_sh = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(frag_sh, 1, (const char* const*)&frag_shader_source, NULL);
    glCompileShader(frag_sh);

    glGetShaderiv(frag_sh, GL_COMPILE_STATUS, &success);
    if (!success) {
        glGetShaderInfoLog(frag_sh, 512, NULL, infoLog);
        printf("Error: fragment shader compilation failed:  %s\n", infoLog);
    }

    shader_program = glCreateProgram();

    glAttachShader(shader_program, vert_sh);
    glAttachShader(shader_program, frag_sh);
    glLinkProgram(shader_program);

    glDeleteShader(vert_sh);
    glDeleteShader(frag_sh);

    glUseProgram(shader_program);

    ratio_uniform = glGetUniformLocation(shader_program, "aspect_ratio");
    uniform_size_x = glGetUniformLocation(shader_program, "size_x");
    uniform_size_y = glGetUniformLocation(shader_program, "size_y");
    uniform_zoom = glGetUniformLocation(shader_program, "zoom");
}

// EXTERN FUNCTIONS
void render() {
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
    glClearColor(0.1, 0.1, 0.1, 1.0);

    glBindVertexArray(vao);
    glUseProgram(shader_program);
    glDrawElementsInstanced(GL_TRIANGLE_STRIP, 6, GL_UNSIGNED_INT, 0, instances_len);
}

void window_resized(int32_t width, int32_t height) {
    glUseProgram(shader_program);
    glUniform1f(ratio_uniform, (float)width / (float)height);
}

void map_resized(float abs_size_x, float abs_size_y) {
    glUseProgram(shader_program);
    glUniform1f(uniform_size_x, abs_size_x);
    glUniform1f(uniform_size_y, abs_size_y);
}

void zoom_changed(float val) {
    glUseProgram(shader_program);
    glUniform1f(uniform_zoom, val);
}

void load_instance_data(void* data, uint32_t len) {
    instances_len = len;
    glBindBuffer(GL_ARRAY_BUFFER, vbo_instances);
    glBufferData(GL_ARRAY_BUFFER, sizeof(float) * 5 * len, data, GL_STATIC_DRAW);
    glBindBuffer(GL_ARRAY_BUFFER, 0);
}

void load_shader(char * str_vert, char * str_frag) {
    if (vert_shader_source != NULL) {
        free(vert_shader_source);
    }
    vert_shader_source = str_vert;
    
    if (frag_shader_source != NULL) {
        free(frag_shader_source);
    }
    frag_shader_source = str_frag;
}

void cleanup() {
    glDeleteBuffers(1, &vbo);
    glDeleteBuffers(1, &vbo_instances);
    glDeleteBuffers(1, &ebo);
    glDeleteVertexArrays(1, &vao);
    free(vert_shader_source);
    free(frag_shader_source);
}

void init_things() {
    glewInit();

    // create buffers
    glGenVertexArrays(1, &vao);
    glGenBuffers(1, &vbo);
    glGenBuffers(1, &ebo);
    glGenBuffers(1, &vbo_instances);

    // load hex coords
    get_hex_verts(verts);

    // upload single hex data and indices
    glBindVertexArray(vao);
    glBindBuffer(GL_ARRAY_BUFFER, vbo);
    glBufferData(GL_ARRAY_BUFFER, verts_len * sizeof(float), verts, GL_STATIC_DRAW);
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);
    glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(indices), indices, GL_STATIC_DRAW);

    // generate program
    generate_program();

    // create vertex attrubutes for single hex
    glBindBuffer(GL_ARRAY_BUFFER, vbo);
    glEnableVertexAttribArray(0);
    glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE, 2 * sizeof(float), NULL);

    //create vertex attributes for instances
    glBindBuffer(GL_ARRAY_BUFFER, vbo_instances);
    glEnableVertexAttribArray(1);
    glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE, 5 * sizeof(float), NULL);
    glVertexAttribDivisor(1, 1);

    glEnableVertexAttribArray(2);
    glVertexAttribPointer(2, 3, GL_FLOAT, GL_FALSE, 5 * sizeof(float), (void*)(2 * sizeof(float)));
    glVertexAttribDivisor(2, 1);
}
