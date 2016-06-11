//
// Created by Jeffrey Yu on 6/11/16.
//

#ifndef PATCHGL_REMOVED_REMOVABLE_H
#define PATCHGL_REMOVED_REMOVABLE_H


#include "Removable.h"

class RemovedRemovable : public Removable {

public:
    virtual bool isRemoved() const override {
        return true;
    }

    virtual void remove() override {
        // Do nothing.
    }
};


#endif //PATCHGL_REMOVED_REMOVABLE_H
