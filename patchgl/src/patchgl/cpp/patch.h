//
// Created by Jeffrey Yu on 4/30/16.
//

#ifndef PATCHGL_PATCH_H
#define PATCHGL_PATCH_H

#include <OpenGL/OpenGL.h>
#include "patchgl.pb.h"

class patch {
public:
    static const wchar_t FULL_BLOCK = L'\u2588';
    wchar_t shape = FULL_BLOCK;

    patch() { }

    patch(const patchgl::BeginPatch_Position &position, const patchgl::BeginPatch_Color &color,
          const wchar_t shape = FULL_BLOCK);

    GLfloat left = -1.f, bottom = -1.f, right = 1.f, top = 1.f;
    GLfloat near = 0.f;
    GLfloat red = 1.f, green = 1.f, blue = 1.f, alpha = 1.f;
};


#endif //PATCHGL_PATCH_H
