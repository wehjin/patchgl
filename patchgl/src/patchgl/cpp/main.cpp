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

    schedulers::run_loop runloop;
    auto mainthread = observe_on_run_loop(runloop);

    screen screen(window, mainthread);

    screen.animation_frame().subscribe([&](double time) {
        screen.setShouldRefresh(true);
    });

    charon charon;
    charon.commands()
            .subscribe_on(observe_on_new_thread())
            .observe_on(mainthread)
            .subscribe([&](Command command) {
                if (command.has_close()) {
                    glfwSetWindowShouldClose(window, GL_TRUE);
                } else if (command.has_begin_patch()) {
                    const BeginPatch &beginPatch = command.begin_patch();
                    const BeginPatch_Position &position = beginPatch.position();
                    const BeginPatch_Color &color = beginPatch.color();
                    unsigned int patchId = beginPatch.patch_id();
                    patch_map[patchId] = patch(position, color);
                    screen.setShouldRefresh(true);
                } else if (command.has_end_patch()) {
                    unsigned int patchId = command.end_patch().patch_id();
                    patch_map.erase(patchId);
                    screen.setShouldRefresh(true);
                }
            });

    while (!glfwWindowShouldClose(window)) {
        while (!runloop.empty() && runloop.peek().when < runloop.now()) {
            runloop.dispatch();
        }
        if (glfwWindowShouldClose(window)) {
            break;
        }
        screen.refresh(patch_map);
        glfwWaitEvents();
    }

    glfwDestroyWindow(window);
    glfwTerminate();
    exit(0);

}
