//
// Created by Jeffrey Yu on 6/18/16.
//

#ifndef PATCHGL_SHADER_H
#define PATCHGL_SHADER_H

#include <string>
#include <fstream>
#include <sstream>
#include <iostream>
#include <GL/glew.h>

class Shader {
public:
    // The program ID
    GLuint program;

    // Constructor reads and builds the shader
    inline Shader(const GLchar *vertexPath, const GLchar *fragmentPath)
            : Shader(getFileContent(vertexPath), getFileContent(fragmentPath)) { }

    inline Shader(const std::string &vertexCode, const std::string &fragmentCode) {
        GLuint vertex, fragment;
        vertex = glCreateShader(GL_VERTEX_SHADER);
        compile(vertex, vertexCode, "VERTEX");
        fragment = glCreateShader(GL_FRAGMENT_SHADER);
        compile(fragment, fragmentCode, "FRAGMENT");
        link(vertex, fragment);
        glDeleteShader(vertex);
        glDeleteShader(fragment);
    }

    inline void use() { glUseProgram(program); }

private:

    inline void link(GLuint vertex, GLuint fragment) {
        program = glCreateProgram();
        glAttachShader(program, vertex);
        glAttachShader(program, fragment);
        glLinkProgram(program);
        GLint success;
        glGetProgramiv(program, GL_LINK_STATUS, &success);
        if (!success) {
            GLchar infoLog[512];
            glGetProgramInfoLog(program, 512, NULL, infoLog);
            std::__1::cerr << "ERROR::SHADER::PROGRAM::LINKING_FAILED\n" << infoLog << std::__1::endl;
        }
    }

    inline static std::string getFileContent(const GLchar *filePath) {
        std::string content;
        std::ifstream file;
        file.exceptions(std::ios_base::badbit);
        try {
            std::stringstream fileStream;
            file.open(filePath);
            fileStream << file.rdbuf();
            file.close();
            content = fileStream.str();
        }
        catch (std::ios_base::failure e) {
            std::cerr << "ERROR::SHADER::FILE_NOT_SUCCESFULLY_READ" << std::endl;
        }
        return content;
    }

    inline static void compile(GLuint id, const std::string &code, const std::string &name) {
        const GLchar *shaderCode = code.c_str();
        glShaderSource(id, 1, &shaderCode, NULL);
        glCompileShader(id);
        GLint success;
        glGetShaderiv(id, GL_COMPILE_STATUS, &success);
        if (!success) {
            GLchar infoLog[512];
            glGetShaderInfoLog(id, 512, NULL, infoLog);
            std::cerr << "ERROR::SHADER::" + name + "::COMPILATION_FAILED\n" << infoLog << std::endl;
        };
    }
};

#endif //PATCHGL_SHADER_H
