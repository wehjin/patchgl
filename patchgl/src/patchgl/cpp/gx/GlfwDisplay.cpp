//
// Created by Jeffrey Yu on 6/11/16.
//

#include <rxcpp/rx.hpp>

#define GLEW_STATIC

#include <GL/glew.h>
#include "GlfwDisplay.h"

using namespace rxcpp;
using namespace rxcpp::sources;
using namespace rxcpp::operators;
using namespace rxcpp::util;
using namespace std;

// Shaders
const GLchar *vertexShaderSource = "#version 330 core\n"
        "layout (location = 0) in vec3 position;\n"
        "layout (location = 1) in vec3 color;\n"
        "out vec3 ourColor;\n"
        "void main()\n"
        "{\n"
        "gl_Position = vec4(position, 1.0);\n"
        "ourColor = color;\n"
        "}\0";
const GLchar *fragmentShaderSource = "#version 330 core\n"
        "in vec3 ourColor;\n"
        "out vec4 color;\n"
        "void main()\n"
        "{\n"
        "color = vec4(ourColor, 1.0f);\n"
        "}\n\0";

void key_callback(GLFWwindow *window, int key, int scancode, int action, int mods) {
    if (key == GLFW_KEY_ESCAPE && action == GLFW_PRESS) {
        glfwSetWindowShouldClose(window, GL_TRUE);
    }
}

void error_callback(int error, const char *description) {
    fputs(description, stderr);
}

schedulers::run_loop runloop;
observe_on_one_worker myWorker = observe_on_run_loop(runloop);

GLFWwindow *createWindow() {
    glfwSetErrorCallback(error_callback);
    if (!glfwInit()) {
        exit(1);
    }
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
    glfwWindowHint(GLFW_RESIZABLE, GL_FALSE);

    GLFWwindow *window = glfwCreateWindow(640, 480, "My Title", nullptr, nullptr);
    if (window == nullptr) {
        std::cout << "Failed to create GLFW window" << std::endl;
        glfwTerminate();
        exit(1);
    }

    glfwMakeContextCurrent(window);
    glewExperimental = GL_TRUE;
    if (glewInit() != GLEW_OK) {
        std::cout << "Failed to initialize GLEW" << std::endl;
        glfwTerminate();
        exit(1);
    }

    glfwSwapInterval(1);
    glfwSetKeyCallback(window, key_callback);
    return window;
}

GlfwDisplay::GlfwDisplay()
        : window(createWindow()), scheduler(myWorker), myScreen(screen(window, myWorker)) {
}

void GlfwDisplay::addPatch(unsigned int patchId, const patch &myPatch) {
    patch_map[patchId] = myPatch;
    refreshWhenIdle();
}

void GlfwDisplay::addPatch(unsigned int patchId, Frame frame, Shape shape, Argb argb) {
    addPatch(patchId, patch(frame, argb, shape));
}

void GlfwDisplay::removePatch(unsigned int patchId) {
    patch_map.erase(patchId);
    refreshWhenIdle();
}

void GlfwDisplay::close() {
    glfwSetWindowShouldClose(window, GL_TRUE);
}

void GlfwDisplay::refreshWhenIdle() {
    myScreen.setShouldRefresh(true);
}

void GlfwDisplay::awaitClose() {

    /*
    myScreen.animation_frame().subscribe([&](double time) {
        myScreen.setShouldRefresh(true);
    });
    */

    // Build and compile our shader program
    // Vertex shader
    GLuint vertexShader = glCreateShader(GL_VERTEX_SHADER);
    glShaderSource(vertexShader, 1, &vertexShaderSource, NULL);
    glCompileShader(vertexShader);
    // Check for compile time errors
    GLint success;
    GLchar infoLog[512];
    glGetShaderiv(vertexShader, GL_COMPILE_STATUS, &success);
    if (!success) {
        glGetShaderInfoLog(vertexShader, 512, NULL, infoLog);
        std::cerr << "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n" << infoLog << std::endl;
    }
    // Fragment shader
    GLuint fragmentShader = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(fragmentShader, 1, &fragmentShaderSource, NULL);
    glCompileShader(fragmentShader);
    // Check for compile time errors
    glGetShaderiv(fragmentShader, GL_COMPILE_STATUS, &success);
    if (!success) {
        glGetShaderInfoLog(fragmentShader, 512, NULL, infoLog);
        std::cerr << "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n" << infoLog << std::endl;
    }
    // Link shaders
    GLuint shaderProgram = glCreateProgram();
    glAttachShader(shaderProgram, vertexShader);
    glAttachShader(shaderProgram, fragmentShader);
    glLinkProgram(shaderProgram);
    // Check for linking errors
    glGetProgramiv(shaderProgram, GL_LINK_STATUS, &success);
    if (!success) {
        glGetProgramInfoLog(shaderProgram, 512, NULL, infoLog);
        std::cerr << "ERROR::SHADER::PROGRAM::LINKING_FAILED\n" << infoLog << std::endl;
    }
    glDeleteShader(vertexShader);
    glDeleteShader(fragmentShader);

    // Set up vertex data (and buffer(s)) and attribute pointers
    ColorSpan colorSpan2 = {0.2f, 0.3f, 0.3f};
    ColorSpan colorSpan = {1.0f, 0.5f, 0.2f};
    PositionSpan bottomLeftPosition = {-0.5f, -0.5f, 0.0f};
    PositionSpan topRightPosition = {0.5f, 0.5f, 0.0f};
    PositionSpan bottomRightPosition = {0.5f, -0.5f, 0.0f};
    PositionSpan topLeftPosition = {-0.5f, 0.5f, 0.0f};
    VertexSpan bottomLeftVertex = {bottomLeftPosition,
                                   colorSpan2};
    VertexSpan bottomRightVertex = {bottomRightPosition,
                                    colorSpan2};
    VertexSpan topLeftVertex = {topLeftPosition,
                                colorSpan};
    VertexSpan topRightVertex = {topRightPosition,
                                 colorSpan};

    screenSpan[1] = (PatchSpan) {
            {bottomLeftVertex, bottomRightVertex, topRightVertex},
            {topRightVertex,   topLeftVertex,     bottomLeftVertex}
    };

    GLuint VBO, VAO;
    glGenVertexArrays(1, &VAO);
    glGenBuffers(1, &VBO);
    // Bind the Vertex Array Object first, then bind and set vertex buffer(s) and attribute pointer(s).
    glBindVertexArray(VAO);

    glBindBuffer(GL_ARRAY_BUFFER, VBO);
    glBufferData(GL_ARRAY_BUFFER, sizeof(screenSpan), &screenSpan, GL_STATIC_DRAW);

    // Position attribute
    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, sizeof(VertexSpan), (GLvoid *) 0);
    glEnableVertexAttribArray(0);

    // Color attribute
    glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE, sizeof(VertexSpan), (GLvoid *) sizeof(PositionSpan));
    glEnableVertexAttribArray(1);

    glBindBuffer(GL_ARRAY_BUFFER, 0);
    glBindVertexArray(0);

    while (!glfwWindowShouldClose(window)) {
        while (!runloop.empty() && runloop.peek().when < runloop.now()) {
            runloop.dispatch();
        }
        if (glfwWindowShouldClose(window)) {
            break;
        }

        float ratio;
        int width, height;

        glfwGetFramebufferSize(window, &width, &height);
        ratio = width / (float) height;

        glViewport(0, 0, width, height);
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

        // Draw our first triangle
        glUseProgram(shaderProgram);
        glBindVertexArray(VAO);
        glDrawArrays(GL_TRIANGLES, 0, vertexSpanCount);
        glBindVertexArray(0);

        // Swap the screen buffers
        glfwSwapBuffers(window);

        //myScreen.refresh(patch_map);
        glfwWaitEvents();
    }

    // Properly de-allocate all resources once they've outlived their purpose
    glDeleteVertexArrays(1, &VAO);
    glDeleteBuffers(1, &VBO);
    glfwDestroyWindow(window);
    glfwTerminate();
}

ShiftDisplay GlfwDisplay::withShift(float horizontal, float vertical) {
    return ShiftDisplay(*this, horizontal, vertical);
}
