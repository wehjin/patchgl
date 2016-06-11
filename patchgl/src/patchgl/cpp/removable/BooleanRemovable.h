//
// Created by Jeffrey Yu on 6/11/16.
//

#ifndef PATCHGL_BOOLEAN_REMOVABLE_H
#define PATCHGL_BOOLEAN_REMOVABLE_H

#include "Removable.h"

class BooleanRemovable : public Removable {
private:
    bool removed = false;

protected:
    virtual void onRemove() = 0;

public:

    virtual bool isRemoved() const override {
        return removed;
    }

    virtual void remove() override {
        if (removed) return;
        removed = true;
        onRemove();
    }
};


#endif //PATCHGL_BOOLEAN_REMOVABLE_H
