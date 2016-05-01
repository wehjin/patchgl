#define GLFW_INCLUDE_GLU

#include <GLFW/glfw3.h>
#include <stdlib.h>
#include <stdio.h>
#include <map>
#include <cstdlib>
#include <ctime>
#include <set>
#include <fstream>
#include <iostream>
#include "patch.h"
#include "patchgl.pb.h"

#include "rxcpp/rx.hpp"

using namespace rxcpp;
using namespace rxcpp::sources;
using namespace rxcpp::operators;
using namespace rxcpp::util;
using namespace std;

void error_callback(int error, const char *description) {
    fputs(description, stderr);
}

void key_callback(GLFWwindow *window, int key, int scancode, int action, int mods) {
    if (key == GLFW_KEY_ESCAPE && action == GLFW_PRESS) {
        glfwSetWindowShouldClose(window, GL_TRUE);
    }
}

int main() {

    srand((unsigned int) std::time(0));

    glfwSetErrorCallback(error_callback);
    if (!glfwInit()) {
        exit(1);
    }

    GLFWwindow *window = glfwCreateWindow(640, 480, "My Title", NULL, NULL);
    if (!window) {
        glfwTerminate();
        exit(1);
    }

    glfwMakeContextCurrent(window);
    glfwSwapInterval(1);
    glfwSetKeyCallback(window, key_callback);

    std::map<int, patch> patch_map;
    patch_map[std::rand()] = patch({-.5f, .5f, .5f, -.5f, 0.f});
    patch_map[std::rand()] = patch({-1.f, 1.f, 0.f, .1f, 0.f});

    patchgl::BeginPatchResponse response;
    unsigned int patchId = (unsigned int) rand();
    cout << "Patch out: " << hex << patchId << endl;
    response.set_patch(patchId);
    fstream output("myfile", ios::out | ios::binary);
    response.SerializeToOstream(&output);
    output.flush();
    output.close();

    fstream input("myfile", ios::in | ios::binary);
    patchgl::BeginPatchResponse responseIn;
    responseIn.ParseFromIstream(&input);
    cout << "Patch: " << hex << responseIn.patch() << endl;

    range(1, 10);
    auto values = from("hello");
    values.subscribe([](const char *s) { cout << "Value: " << s << endl; });

    while (!glfwWindowShouldClose(window)) {
        float ratio;
        int width, height;

        glfwGetFramebufferSize(window, &width, &height);
        ratio = width / (float) height;

        glViewport(0, 0, width, height);
        glClear(GL_COLOR_BUFFER_BIT);

        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();
        glOrtho(-ratio, ratio, -1.f, 1.f, 1.f, -1.f);

        glMatrixMode(GL_MODELVIEW);
        glLoadIdentity();
        glRotatef((float) glfwGetTime() * 50.f, 0.f, 0.f, 1.f);

        glBegin(GL_QUADS);
        for (auto &entry : patch_map) {
            auto &patch = entry.second;
            glColor3f(1.f, 0.f, 0.f);
            glVertex3f(patch.left, patch.bottom, patch.near);
            glColor3f(0.f, 1.f, 0.f);
            glVertex3f(patch.left, patch.top, patch.near);
            glColor3f(0.f, 1.f, 0.f);
            glVertex3f(patch.right, patch.top, patch.near);
            glColor3f(0.f, 0.f, 1.f);
            glVertex3f(patch.right, patch.bottom, patch.near);
        }
        glEnd();

        glfwSwapBuffers(window);
        glfwPollEvents();
    }

    glfwDestroyWindow(window);
    glfwTerminate();
    exit(0);

}
