//
// Created by Jeffrey Yu on 4/30/16.
//

#include <GLFW/glfw3.h>
#include <chrono>
#include "screen.h"

using namespace std::chrono;
using namespace rxcpp::sources;

screen::screen(GLFWwindow *window, observe_on_one_worker &mainthread) : window(window), mainthread(mainthread) {
}

void screen::refresh(std::map<unsigned int, patch> &patch_map) {
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
}

observable<double> screen::animation_frame() {
    return interval(milliseconds(21))
            .map([](int _) { return glfwGetTime(); })
            .map([](double time) {
                glfwPostEmptyEvent();
                return time;
            })
            .subscribe_on(observe_on_new_thread())
            .observe_on(mainthread);
}

