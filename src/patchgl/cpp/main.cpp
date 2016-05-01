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
#include "screen.h"
#include "charon.h"

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

    std::map<unsigned int, patch> patch_map;

    charon charon;

    charon.begin_patch_requests().subscribe([&](patchgl::BeginPatch beginPatch) {
        auto &position = beginPatch.position();
        unsigned int patchId = (unsigned int) rand();
        patch_map[patchId] = patch(position.left(), position.top(), position.right(),
                                   position.bottom(), position.near());

        patchgl::BeginPatchResponse response;
        response.set_patch(patchId);
    });

    schedulers::run_loop runloop;
    auto mainthread = observe_on_run_loop(runloop);

    screen screen(window, mainthread);

    screen.animation_frame().subscribe([&](double time) {
        screen.refresh(patch_map);
    });

    while (!glfwWindowShouldClose(window)) {
        while (!runloop.empty() && runloop.peek().when < runloop.now()) {
            runloop.dispatch();
        }
        glfwWaitEvents();
    }

    glfwDestroyWindow(window);
    glfwTerminate();
    exit(0);

}
