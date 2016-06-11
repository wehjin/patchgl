//
// Created by Jeffrey Yu on 6/11/16.
//

#ifndef PATCHGL_REMOVABLE_H
#define PATCHGL_REMOVABLE_H


class Removable {
public:
    virtual bool isRemoved() const = 0;

    virtual void remove() = 0;
};


#endif //PATCHGL_REMOVABLE_H
