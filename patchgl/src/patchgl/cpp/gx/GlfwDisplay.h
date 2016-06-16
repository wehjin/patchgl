//
// Created by Jeffrey Yu on 6/11/16.
//

#ifndef PATCHGL_GL_DISPLAY_H
#define PATCHGL_GL_DISPLAY_H

#include <GLFW/glfw3.h>
#include <cstdlib>
#include <cstdio>
#include "Display.h"
#include "../removable/EmptyRemovable.h"
#include "../screen.h"
#include "ShiftDisplay.h"

class GlfwDisplay final : Display {

public:

    GLFWwindow *window;
    screen myScreen;
    const observe_on_one_worker &scheduler;

    GlfwDisplay();

    void close();

    void awaitClose();

    void addPatch(unsigned int patchId, const patch &myPatch);

    virtual void addPatch(unsigned int patchId, Frame frame1, Shape shape1, Argb argb1);

    virtual void removePatch(unsigned int patchId);

    ShiftDisplay withShift(float horizontal, float vertical);

private:
    std::map<unsigned int, patch> patch_map;

    void refreshWhenIdle();

    typedef struct {
        GLfloat x, y, z;
    } PositionSpan;

    typedef struct {
        GLfloat r, g, b;
    } ColorSpan;

    typedef struct {
        PositionSpan position;
        ColorSpan color;
    } VertexSpan;

#pragma pack(1)
    typedef struct {
        VertexSpan bl, br, tr;
    } BottomRightTriangle;

#pragma pack(1)
    typedef struct {
        VertexSpan tr, tl, bl;
    } TopLeftTriangle;

#pragma pack(1)
    typedef struct {
        BottomRightTriangle bottomRight;
        TopLeftTriangle topLeft;
    } PatchSpan;

    static const unsigned int patchSpanCount = 1000;
    static const unsigned int triangleSpanCount = patchSpanCount * 2;
    static const unsigned int vertexSpanCount = triangleSpanCount * 3;
    PatchSpan screenSpan[patchSpanCount];
};


#endif //PATCHGL_GL_DISPLAY_H
