//
// Created by Jeffrey Yu on 5/1/16.
//

#ifndef PATCHGL_CHARON_H
#define PATCHGL_CHARON_H

#include "patchgl.pb.h"
#include "rxcpp/rx.hpp"

using namespace rxcpp;
using namespace std;

class charon {
public:
    observable<patchgl::BeginPatch> begin_patch_requests();
};


#endif //PATCHGL_CHARON_H
