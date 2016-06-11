//
// Created by Jeffrey Yu on 6/11/16.
//

#ifndef PATCHGL_GL_DISPLAY_H
#define PATCHGL_GL_DISPLAY_H


#include <GLFW/glfw3.h>
#include <cstdlib>
#include <cstdio>
#include "Display.h"
#include "../removable/RemovedRemovable.h"
#include "../screen.h"


class GlfwDisplay final : Display, public BooleanRemovable {
public:
    GLFWwindow *window;
    screen myScreen;
    const observe_on_one_worker &scheduler;

    GlfwDisplay();

    void close();

    void awaitClose();

    virtual Removable addPatch(Frame frame1, Shape shape1, Argb argb1);

    void addPatch(unsigned int patchId, const patch &myPatch);

    void removePatch(unsigned int patchId);

private:
    std::map<unsigned int, patch> patch_map;

    void refreshWhenIdle();

protected:
    virtual void onRemove();
};


#endif //PATCHGL_GL_DISPLAY_H
