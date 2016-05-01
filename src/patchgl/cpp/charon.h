//
// Created by Jeffrey Yu on 5/1/16.
//

#ifndef PATCHGL_CHARON_H
#define PATCHGL_CHARON_H

#include <GLFW/glfw3.h>
#include "patchgl.pb.h"
#include "rxcpp/rx.hpp"

using namespace rxcpp;
using namespace rxcpp::sources;
using namespace rxcpp::operators;
using namespace rxcpp::util;
using namespace std;

class charon {
public:

    observable<patchgl::BeginPatch> begin_patch_requests();
};


#endif //PATCHGL_CHARON_H
