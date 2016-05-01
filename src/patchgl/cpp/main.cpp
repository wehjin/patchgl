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
    patch_map[rand()] = patch({-.5f, .5f, .5f, -.5f, 0.f});
    patch_map[rand()] = patch({-1.f, 1.f, 0.f, .1f, 0.f});

    patchgl::BeginPatchResponse response;
    unsigned int patchId = (unsigned int) rand();
    cout << "Patch out: " << hex << patchId << endl;
    response.set_patch(patchId);
    fstream output("myfile", ios::out | ios::binary);
    response.SerializeToOstream(&output);
    output.flush();
    output.close();

    {
        patchgl::BeginPatch beginPatch;
        beginPatch.mutable_color()->set_red(1.f);
        beginPatch.mutable_color()->set_green(0.f);
        beginPatch.mutable_color()->set_blue(0.f);
        beginPatch.mutable_position()->set_left(0.f);
        beginPatch.mutable_position()->set_right(300.f);
        beginPatch.mutable_position()->set_top(10.f);
        beginPatch.mutable_position()->set_bottom(100.f);
        beginPatch.
        fstream outputBeginPatch("beginpatch", ios::out | ios::binary);
        beginPatch.SerializeToOstream(&outputBeginPatch);
    }

    fstream input("myfile", ios::in | ios::binary);
    patchgl::BeginPatchResponse responseIn;
    responseIn.ParseFromIstream(&input);
    cout << "Patch: " << hex << responseIn.patch() << endl;

    range(1, 10);
    auto values = from("hello");
    values.subscribe([](const char *s) { cout << "Value: " << s << endl; });

    screen screen(window);
    while (!glfwWindowShouldClose(window)) {
        screen.refresh(patch_map);
        glfwWaitEvents();
    }

    glfwDestroyWindow(window);
    glfwTerminate();
    exit(0);

}
