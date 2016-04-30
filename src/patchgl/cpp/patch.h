//
// Created by Jeffrey Yu on 4/30/16.
//

#ifndef PATCHGL_PATCH_H
#define PATCHGL_PATCH_H


#include <OpenGL/OpenGL.h>

class patch {
public:
    GLfloat left;
    GLfloat top;
    GLfloat right;
    GLfloat bottom;
    GLfloat near;

    patch() { }

    patch(GLfloat left, GLfloat top, GLfloat right, GLfloat bottom, GLfloat near) : left(left), top(top), right(right),
                                                                                    bottom(bottom), near(near) { }
};


#endif //PATCHGL_PATCH_H
