#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#define GLEW_STATIC
#include <GL/glew.h>
#include <stdint.h>

// RUST FUNCTIONS
extern void get_hex_verts(float* verts);
extern void get_instance_data(float* instances, void* map, uint32_t x, uint32_t y);

// CONSTANTS
const uint32_t indices[] = {
    5, 4, 0, 3, 1, 2
};

// 6 verts with 2 coords each
const int32_t verts_len = 12; 

// GLOBALS
char * vert_shader_source;
char * frag_shader_source;
uint32_t vao, vbo, ebo, vbo_instances, ratio_uniform, uniform_size_x, uniform_size_y, shader_program;

float verts[12];

float * instances;

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
}

// EXTERN FUNCTIONS
void render(uint32_t x, uint32_t y) {
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
    glClearColor(0.1, 0.1, 0.1, 1.0);

    glBindVertexArray(vao);
    glDrawElementsInstanced(GL_TRIANGLE_STRIP, 6, GL_UNSIGNED_INT, 0, x * y);
}

void window_resized(int32_t width, int32_t height, float abs_size_x, float abs_size_y) {
    glUniform1f(ratio_uniform, (float)width / (float)height);
    glUniform1f(uniform_size_x, abs_size_x);
    glUniform1f(uniform_size_y, abs_size_y);
}

void load_instance_data(void* map, uint32_t x, uint32_t y) {
    get_instance_data(instances, map, x, y);
    glBindBuffer(GL_ARRAY_BUFFER, vbo_instances);
    glBufferData(GL_ARRAY_BUFFER, sizeof(float) * 5 * x * y, instances, GL_STATIC_DRAW);
    glBindBuffer(GL_ARRAY_BUFFER, 0);
}

void load_shader(char * str_vert, char * str_frag) {
    vert_shader_source = str_vert;
    frag_shader_source = str_frag;
}

void cleanup() {
    glDeleteBuffers(1, &vbo);
    glDeleteBuffers(1, &vbo_instances);
    glDeleteBuffers(1, &ebo);
    glDeleteVertexArrays(1, &vao);
}

void init_things(uint64_t len) {
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


    // upload instance data
    instances = malloc(len * sizeof(float) * 5);

    glBindVertexArray(vbo_instances);
    glBindBuffer(GL_ARRAY_BUFFER, vbo_instances);
    glBufferData(GL_ARRAY_BUFFER, sizeof(float) * 5 * len, instances, GL_STATIC_DRAW);
    glBindBuffer(GL_ARRAY_BUFFER, 0);

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

