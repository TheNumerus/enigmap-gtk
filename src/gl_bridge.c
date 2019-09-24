#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#define GLEW_STATIC
#include <GL/glew.h>

extern float * get_hex_verts(float* verts);

char * vert_shader_source;
char * frag_shader_source;
unsigned int vao, vbo, ebo;

float verts[12];

unsigned int indices[] = {
    5, 4, 0, 3, 1, 2
};

void render(int width, int height) {
    glClear(GL_COLOR_BUFFER_BIT);
    glClear(GL_DEPTH_BUFFER_BIT);
    glClearColor(0.1, 0.1, 0.1, 1.0);
    
    glBindVertexArray(vao);
    glDrawElements(GL_TRIANGLE_STRIP, 6, GL_UNSIGNED_INT, 0);
}

void load_shader(char * str_vert, char * str_frag) {
    int len = strlen(str_vert);
    vert_shader_source = (char *)malloc(len);
    strncpy(vert_shader_source, str_vert, len);

    len = strlen(str_frag);
    frag_shader_source = (char *)malloc(len);
    strncpy(frag_shader_source, str_frag, len);
}


void init_things() {
    glewInit();
    int verts_len = 12;

    glGenVertexArrays(1, &vao);
    glGenBuffers(1, &vbo);
    glGenBuffers(1, &ebo);
    glBindVertexArray(vao);
    glBindBuffer(GL_ARRAY_BUFFER, vbo);
    get_hex_verts(verts);
    glBufferData(GL_ARRAY_BUFFER, verts_len * sizeof(float), verts, GL_STATIC_DRAW);
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);
    glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(indices), indices, GL_STATIC_DRAW);

    unsigned int vert_sh;
    vert_sh = glCreateShader(GL_VERTEX_SHADER);
    glShaderSource(vert_sh, 1, &vert_shader_source, NULL);
    glCompileShader(vert_sh);

    int success;
    char infoLog[512];
    glGetShaderiv(vert_sh, GL_COMPILE_STATUS, &success);
    if (!success) {
        glGetShaderInfoLog(vert_sh, 512, NULL, infoLog);
        printf("ERROR::SHADER::VERTEX::COMPILATION_FAILED, %s\n",infoLog);
    }

    unsigned int frag_sh;
    frag_sh = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(frag_sh, 1, &frag_shader_source, NULL);
    glCompileShader(frag_sh);


    glGetShaderiv(frag_sh, GL_COMPILE_STATUS, &success);
    if (!success)
    {
        glGetShaderInfoLog(frag_sh, 512, NULL, infoLog);
        printf("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED, %s\n", infoLog);
    }

    unsigned int shaderProgram;
    shaderProgram = glCreateProgram();

    glAttachShader(shaderProgram, vert_sh);
    glAttachShader(shaderProgram, frag_sh);
    glLinkProgram(shaderProgram);

    glUseProgram(shaderProgram);

    glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE, 2 * sizeof(float), NULL);
    glEnableVertexAttribArray(0);
}

void destroy_shader() {
    free(vert_shader_source);
    free(frag_shader_source);
}
