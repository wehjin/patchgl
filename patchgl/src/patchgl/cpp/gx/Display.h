//
// Created by Jeffrey Yu on 6/11/16.
//

#ifndef PATCHGL_GX_SCREEN_H
#define PATCHGL_GX_SCREEN_H


#include "../removable/Removable.h"
#include "Shape.h"
#include "Frame.h"
#include "Argb.h"
#include "../removable/BooleanRemovable.h"
#include "../removable/EmptyRemovable.h"

class Display {
public:
    virtual void addPatch(unsigned int patchId, Frame, Shape, Argb) = 0;

    virtual void removePatch(unsigned int patchId) = 0;
};


#endif //PATCHGL_GX_SCREEN_H
