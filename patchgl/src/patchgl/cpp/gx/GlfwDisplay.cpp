//
// Created by Jeffrey Yu on 6/11/16.
//

#include <rxcpp/rx.hpp>

#define GLEW_STATIC

#include <GL/glew.h>
#include <ft2build.h>
#include FT_FREETYPE_H
#include "GlfwDisplay.h"
#include "Shader.h"
#include "../data/vertex_glsl.h"
#include "../data/fragment_glsl.h"
#include "Scribe.h"

using namespace rxcpp;
using namespace rxcpp::sources;
using namespace rxcpp::operators;
using namespace rxcpp::util;
using namespace std;

size_t positionOffset = 0;
size_t colorOffset = positionOffset + sizeof(PositionSpan);
size_t textureCoordinateOffset = colorOffset + sizeof(ColorSpan);
size_t textureUnitOffset = textureCoordinateOffset + sizeof(TextureCoordinateSpan);

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

ColorSpan emptyColor = {0.f, 0.f, 0.f};
PositionSpan emptyPosition = {0.f, 0.f, 0.f};
TextureCoordinateSpan bottomLeftTextureCoordinate = {0.f, 0.f};
TextureCoordinateSpan bottomRightTextureCoordinate = {1.f, 0.f};
TextureCoordinateSpan topLeftTextureCoordinate = {0.f, 1.f};
TextureCoordinateSpan topRightTextureCoordinate = {1.f, 1.f};
int emptyTextureUnit = -1;
VertexSpan emptyVertex = {emptyPosition,
                          emptyColor,
                          bottomLeftTextureCoordinate,
                          emptyTextureUnit};
PatchSpan emptyPatch = {{emptyVertex, emptyVertex, emptyVertex},
                        {emptyVertex, emptyVertex, emptyVertex}};

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
        : window(createWindow()),
          scheduler(myWorker),
          myScreen(screen(window, myWorker)) {

    for (unsigned int i = 0; i < patchSpanCount; i++) {
        freeStack[i] = i + 1;
        screenSpan[i] = emptyPatch;
    }

    glEnable(GL_DEPTH_TEST);
    glDepthFunc(GL_LESS);
    glEnable(GL_BLEND);
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
}

void GlfwDisplay::addPatch(unsigned int patchId, const patch &myPatch) {
    if (freeStackTop >= patchSpanCount)
        return;

    unsigned int spanIndex;
    if (patch_map.find(patchId) == patch_map.end()) {
        spanIndex = freeStackTop;
        freeStackTop = freeStack[freeStackTop];
        patch_map[patchId] = spanIndex;
    } else {
        spanIndex = patch_map[patchId];
    }

    ColorSpan colorSpan = {myPatch.red, myPatch.green, myPatch.blue};
    PositionSpan bottomLeftPosition = {myPatch.left, myPatch.bottom, (myPatch.near)};
    PositionSpan topRightPosition = {myPatch.right, myPatch.top, (myPatch.near)};
    PositionSpan bottomRightPosition = {myPatch.right, myPatch.bottom, (myPatch.near)};
    PositionSpan topLeftPosition = {myPatch.left, myPatch.top, (myPatch.near)};
    GLint textureUnit = myPatch.shape == patch::FULL_BLOCK ? emptyTextureUnit : 0;
    VertexSpan bottomLeftVertex = {bottomLeftPosition, colorSpan, bottomLeftTextureCoordinate, textureUnit};
    VertexSpan bottomRightVertex = {bottomRightPosition, colorSpan, bottomRightTextureCoordinate, textureUnit};
    VertexSpan topLeftVertex = {topLeftPosition, colorSpan, topLeftTextureCoordinate, textureUnit};
    VertexSpan topRightVertex = {topRightPosition, colorSpan, topRightTextureCoordinate, textureUnit};
    screenSpan[spanIndex].bottomRight.bl = bottomLeftVertex;
    screenSpan[spanIndex].bottomRight.br = bottomRightVertex;
    screenSpan[spanIndex].bottomRight.tr = topRightVertex;
    screenSpan[spanIndex].topLeft.tr = topRightVertex;
    screenSpan[spanIndex].topLeft.tl = topLeftVertex;
    screenSpan[spanIndex].topLeft.bl = bottomLeftVertex;
    refreshWhenIdle();
}

void GlfwDisplay::addPatch(unsigned int patchId, Frame frame, Shape shape, Argb argb) {
    addPatch(patchId, patch(frame, argb, shape));
}

void GlfwDisplay::removePatch(unsigned int patchId) {
    if (patch_map.find(patchId) == patch_map.end())
        return;

    unsigned int spanIndex = patch_map[patchId];
    patch_map.erase(patchId);
    screenSpan[spanIndex] = emptyPatch;
    freeStack[spanIndex] = freeStackTop;
    freeStackTop = spanIndex;
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

    Scribe scribe;


    // Build and compile our shader program
    Shader shader(std::string((const char *) vertex_glsl, vertex_glsl_len),
                  std::string((const char *) fragment_glsl, fragment_glsl_len));

    // Set up vertex data (and buffer(s)) and attribute pointers
    GLuint VBO, VAO;
    glGenVertexArrays(1, &VAO);
    glGenBuffers(1, &VBO);
    glBindVertexArray(VAO);
    glBindBuffer(GL_ARRAY_BUFFER, VBO);
    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, sizeof(VertexSpan), (GLvoid *) positionOffset);
    glEnableVertexAttribArray(0);
    glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE, sizeof(VertexSpan), (GLvoid *) colorOffset);
    glEnableVertexAttribArray(1);
    glVertexAttribPointer(2, 2, GL_FLOAT, GL_TRUE, sizeof(VertexSpan), (GLvoid *) textureCoordinateOffset);
    glEnableVertexAttribArray(2);
    glVertexAttribPointer(3, 1, GL_INT, GL_FALSE, sizeof(VertexSpan), (GLvoid *) textureUnitOffset);
    glEnableVertexAttribArray(3);

    GLuint texture;
    glGenTextures(1, &texture);
    glBindTexture(GL_TEXTURE_2D, texture);
    glTexImage2D(GL_TEXTURE_2D, 0, GL_RED, scribe.getWidth(), scribe.getHeight(), 0, GL_RED, GL_UNSIGNED_BYTE,
                 scribe.getImage());
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_BORDER);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_BORDER);
    glClearColor(0.2f, 0.3f, 0.3, 1.0f);

    while (!glfwWindowShouldClose(window)) {
        while (!runloop.empty() && runloop.peek().when < runloop.now()) {
            runloop.dispatch();
        }
        if (glfwWindowShouldClose(window)) {
            break;
        }

        int width, height;
        glfwGetFramebufferSize(window, &width, &height);
        float ratio = width / (float) height;
        glViewport(0, 0, width, height);
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        shader.use();
        glBufferData(GL_ARRAY_BUFFER, sizeof(screenSpan), &screenSpan, GL_DYNAMIC_DRAW);
        glDrawArrays(GL_TRIANGLES, 0, vertexSpanCount);
        glfwSwapBuffers(window);
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
