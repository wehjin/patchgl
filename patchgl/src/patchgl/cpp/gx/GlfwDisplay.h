//
// Created by Jeffrey Yu on 6/11/16.
//

#ifndef PATCHGL_GL_DISPLAY_H
#define PATCHGL_GL_DISPLAY_H

#include <GLFW/glfw3.h>
#include <cstdlib>
#include <cstdio>
#include "Display.h"
#include "rxcpp/rx.hpp"
#include "../patch.h"
#include "../removable/EmptyRemovable.h"
#include "ShiftDisplay.h"
#include "Scribe.h"
#include "Span.h"

using namespace rxcpp;
using namespace rxcpp::sources;
using namespace rxcpp::operators;
using namespace rxcpp::util;


class GlfwDisplay final : Display {

public:

    GLFWwindow *window;
    const observe_on_one_worker &scheduler;
    bool shouldPostRedrawEvent = false;

    GlfwDisplay();

    void close();

    void awaitClose();

    void addPatch(unsigned int patchId, const patch &myPatch);

    virtual void addPatch(unsigned int patchId, Frame frame1, Shape shape1, Argb argb1);

    virtual void removePatch(unsigned int patchId);

    ShiftDisplay withShift(float horizontal, float vertical);

private:
    std::map<unsigned int, unsigned int> patch_map;

    void refreshWhenIdle();

    static const unsigned int patchSpanCount = 1000;
    static const unsigned int triangleSpanCount = patchSpanCount * 2;
    static const unsigned int vertexSpanCount = triangleSpanCount * 3;
    PatchSpan screenSpan[patchSpanCount];
    unsigned int freeStack[patchSpanCount];
    unsigned int freeStackTop = 0;
    Scribe scribe;
};


#endif //PATCHGL_GL_DISPLAY_H
