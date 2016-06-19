//
// Created by Jeffrey Yu on 5/1/16.
//

#ifndef PATCHGL_CHARON_H
#define PATCHGL_CHARON_H

#include "data/patchgl.pb.h"
#include "rxcpp/rx.hpp"

using namespace rxcpp;
using namespace std;
using namespace patchgl;

class charon {
public:
    observable<Command> commands();
};


#endif //PATCHGL_CHARON_H
