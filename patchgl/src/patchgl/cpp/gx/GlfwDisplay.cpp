//
// Created by Jeffrey Yu on 6/11/16.
//

#include <rxcpp/rx.hpp>
#include "GlfwDisplay.h"

using namespace rxcpp;
using namespace rxcpp::sources;
using namespace rxcpp::operators;
using namespace rxcpp::util;
using namespace std;

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

    GLFWwindow *window = glfwCreateWindow(640, 480, "My Title", NULL, NULL);
    if (!window) {
        glfwTerminate();
        exit(1);
    }

    glfwMakeContextCurrent(window);
    glfwSwapInterval(1);
    glfwSetKeyCallback(window, key_callback);
    return window;
}

GlfwDisplay::GlfwDisplay()
        : window(createWindow()), scheduler(myWorker), myScreen(screen(window, myWorker)) {
}

void GlfwDisplay::onRemove() {
    glfwDestroyWindow(window);
    glfwTerminate();
}

std::shared_ptr<Removable> GlfwDisplay::addPatch(unsigned int id, Frame frame, Shape shape, Argb argb) {
    addPatch(id, patch(frame, argb, shape));
    std::shared_ptr<Removable> ptr(new PatchRemovable(this, id));
    return ptr;
}

void GlfwDisplay::addPatch(unsigned int patchId, const patch &myPatch) {
    patch_map[patchId] = myPatch;
    refreshWhenIdle();
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

    while (!glfwWindowShouldClose(window)) {
        while (!runloop.empty() && runloop.peek().when < runloop.now()) {
            runloop.dispatch();
        }
        if (glfwWindowShouldClose(window)) {
            break;
        }
        myScreen.refresh(patch_map);
        glfwWaitEvents();
    }
}

ShiftDisplay GlfwDisplay::withShift(float horizontal, float vertical) {
    return ShiftDisplay(*this, horizontal, vertical);
}
