//
// Created by Jeffrey Yu on 4/30/16.
//

#include <GLFW/glfw3.h>
#include <chrono>
#include "screen.h"
#include "font.h"

using namespace std::chrono;
using namespace rxcpp::sources;

screen::screen(GLFWwindow *window, observe_on_one_worker &mainthread)
        : window(window), mainthread(mainthread), font(Font_ttf, Font_ttf_len) {
    FT_Error error = font.Error();
    if (error) {
        throw new bad_alloc();
    }
    font.FaceSize(32);
    glDepthFunc(GL_LESS);
    glEnable(GL_DEPTH_TEST);
}

observable<double> screen::animation_frame() {
    return interval(milliseconds(21))
            .map([](int _) { return glfwGetTime(); })
            .subscribe_on(observe_on_new_thread())
            .observe_on(mainthread);
}

void screen::refresh(std::map<unsigned int, patch> &patch_map) {
    if (!shouldRefresh) {
        return;
    }
    shouldRefresh = false;

    float ratio;
    int width, height;

    glfwGetFramebufferSize(window, &width, &height);
    ratio = width / (float) height;

    glViewport(0, 0, width, height);
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

    glMatrixMode(GL_PROJECTION);
    glLoadIdentity();
    glOrtho(-ratio, ratio, -1.f, 1.f, -1.f, 1.f);

    glMatrixMode(GL_MODELVIEW);
    glLoadIdentity();
    //glRotatef((float) glfwGetTime() * 50.f, 0.f, 0.f, 1.f);

    glBegin(GL_QUADS);
    for (auto &entry : patch_map) {
        patch &patch = entry.second;
        if (patch.shape == patch::FULL_BLOCK) {
            glColor4f(patch.red, patch.green, patch.blue, patch.alpha);
            glVertex3f(patch.left, patch.bottom, patch.near);
            glVertex3f(patch.left, patch.top, patch.near);
            glVertex3f(patch.right, patch.top, patch.near);
            glVertex3f(patch.right, patch.bottom, patch.near);
        }
    }
    glEnd();

    for (auto &entry : patch_map) {
        patch &patch = entry.second;
        if (patch.shape != patch::FULL_BLOCK) {
            glColor4f(patch.red, patch.green, patch.blue, patch.alpha);
            glPushMatrix();
            glTranslatef(patch.left, patch.bottom, patch.near);
            glScalef(0.124f * (patch.right - patch.left) / 2.f, 0.1f * (patch.top - patch.bottom) / 2.f, 1.f);
            font.Render(wstring(&patch.shape, 1).data());
            glPopMatrix();
        }
    }

    glfwSwapBuffers(window);
}

void screen::setShouldRefresh(bool shouldRefresh) {
    this->shouldRefresh = shouldRefresh;
    if (shouldRefresh) {
        glfwPostEmptyEvent();
    }
}



