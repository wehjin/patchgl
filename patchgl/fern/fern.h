//
// Created by Jeffrey Yu on 4/30/16.
//

#ifndef PATCHGL_FERN_H
#define PATCHGL_FERN_H

#include "frond.h"

class fern {

    const int frond_start = 2;
    int frond_count;
    frond *fronds;
    frond &pool;
    frond &root;

public:
    const int INVALID_ID = 0;
    const int ROOT_ID = 1;

    int size;

    fern(int size);

    virtual ~fern();

    int pool_size();

    int add_child(int parent_id, const void *payload);

    const void *payload_from(int id);

    int parent_from(int id);

    bool are_siblings(int id1, int id2);

    bool is_child(int parent_id, int id);

    bool is_descendant(int ancestor_id, int id);
};


#endif //PATCHGL_FERN_H
