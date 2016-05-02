//
// Created by Jeffrey Yu on 4/30/16.
//

#ifndef PATCHGL_WINDOW_H
#define PATCHGL_WINDOW_H

#include <map>
#include "patch.h"
#include "rxcpp/rx.hpp"

using namespace rxcpp;
using namespace rxcpp::sources;
using namespace rxcpp::operators;
using namespace rxcpp::util;
using namespace std;

class screen {

    struct GLFWwindow *window;
    observe_on_one_worker &mainthread;
    bool shouldRefresh = true;

public:
    screen(GLFWwindow *window, observe_on_one_worker &mainthread);

    void refresh(std::map<unsigned int, patch> &patch_map);

    observable<double> animation_frame();

    void setShouldRefresh(bool shouldRefresh);
};


#endif //PATCHGL_WINDOW_H
