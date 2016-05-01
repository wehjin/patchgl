//
// Created by Jeffrey Yu on 4/30/16.
//

#ifndef PATCHGL_WINDOW_H
#define PATCHGL_WINDOW_H

#include <map>
#include "patch.h"

class screen {

    struct GLFWwindow *window;

public:
    screen(struct GLFWwindow *window);

    void refresh(std::map<unsigned int, patch> &patch_map);
};


#endif //PATCHGL_WINDOW_H
