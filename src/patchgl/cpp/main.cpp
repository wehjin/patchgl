#define GLFW_INCLUDE_GLU

#include <GLFW/glfw3.h>
#include <stdlib.h>
#include <stdio.h>
#include "patch.h"

void error_callback(int error, const char *description) {
    fputs(description, stderr);
}

void key_callback(GLFWwindow *window, int key, int scancode, int action, int mods) {
    if (key == GLFW_KEY_ESCAPE && action == GLFW_PRESS) {
        glfwSetWindowShouldClose(window, GL_TRUE);
    }
}

int main() {

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

    const int patchCount = 2;
    patch patch_list[patchCount] = {
            {-.5f, .5f, .5f, -.5f, 0.f},
            {-1.f, 1.f, 0.f, .1f,  0.f},
    };

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
        patch *patch = patch_list;
        for (int i = 0; i < patchCount; i++) {
            glColor3f(1.f, 0.f, 0.f);
            glVertex3f(patch->left, patch->bottom, patch->near);
            glColor3f(0.f, 1.f, 0.f);
            glVertex3f(patch->left, patch->top, patch->near);
            glColor3f(0.f, 1.f, 0.f);
            glVertex3f(patch->right, patch->top, patch->near);
            glColor3f(0.f, 0.f, 1.f);
            glVertex3f(patch->right, patch->bottom, patch->near);
            patch++;
        }
        glEnd();

        glfwSwapBuffers(window);
        glfwPollEvents();
    }

    glfwDestroyWindow(window);
    glfwTerminate();
    exit(0);

}
