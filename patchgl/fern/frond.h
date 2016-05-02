//
// Created by Jeffrey Yu on 4/30/16.
//

#ifndef PATCHGL_FRONT_H
#define PATCHGL_FRONT_H


class frond {
public:
    int parent;
    int child;
    int previous;
    int next;
    const void *payload;
};


#endif //PATCHGL_FRONT_H
