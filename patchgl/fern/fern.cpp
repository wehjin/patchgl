//
// Created by Jeffrey Yu on 4/30/16.
//

#include "fern.h"


fern::fern(int size) : size(size), frond_count(size + frond_start), fronds(new frond[frond_count]), pool(fronds[0]),
                       root(fronds[1]) {
    int previous = 0;
    for (int i = frond_start; i < frond_count; i++) {
        frond &frond = fronds[i];
        frond.parent = 0;
        frond.child = 0;
        frond.previous = previous;
        frond.next = 0;
        fronds[previous].next = i;
        previous = i;
    }
    pool.parent = 0;
    pool.child = 0;
    pool.next = 0;
    pool.next = frond_start;
    pool.previous = previous;

    root.parent = 1;
    root.child = 0;
    root.next = 0;
    root.previous = 0;
}

fern::~fern() {
    delete[] fronds;
}

int fern::add_child(int parent_id, const void *payload) {
    // Parent can be any descendant of root
    if (parent_id == INVALID_ID || !is_descendant(ROOT_ID, parent_id)) {
        return INVALID_ID;
    }
    frond &parent = fronds[parent_id];

    int out_id = pool.next;
    if (out_id == INVALID_ID) {
        return INVALID_ID;
    }

    // Extract from pool
    frond &out = fronds[out_id];
    if (out.child != INVALID_ID) {
        // TODO Move child to pool instead of failing.
        return INVALID_ID;
    }
    if (out.next != INVALID_ID) {
        fronds[out.next].previous = INVALID_ID;
    }
    pool.next = out.next;
    out.next = INVALID_ID;
    out.previous = INVALID_ID;
    out.payload = payload;

    // Insert into parent forest.
    out.parent = parent_id;
    out.next = parent.child;
    if (out.next != INVALID_ID) {
        fronds[out.next].previous = out_id;
    }
    parent.child = out_id;

    return out_id;
}

const void *fern::payload_from(int id) {
    return fronds[id].payload;
}

int fern::parent_from(int id) {
    return fronds[id].parent;
}


int fern::pool_size() {
    int count = 0;
    int nextIndex = pool.next;
    while (nextIndex != 0) {
        count++;
        frond &frond = fronds[nextIndex];
        nextIndex = frond.next;
        // TODO: Account for children.
    }
    return count;
}

bool fern::are_siblings(int id1, int id2) {
    int next_id = fronds[id1].next;
    while (next_id != INVALID_ID) {
        if (id2 == next_id) {
            return true;
        }
        next_id = fronds[next_id].next;
    }

    int previous_id = fronds[id1].previous;
    while (previous_id != INVALID_ID) {
        if (id2 == previous_id) {
            return true;
        }
        previous_id = fronds[previous_id].previous;
    }
    return false;
}

bool fern::is_child(int parent_id, int id) {
    int child_id = fronds[parent_id].child;
    if (child_id == INVALID_ID) {
        return false;
    }
    if (child_id == id) {
        return true;
    }
    return are_siblings(child_id, id);
}

bool fern::is_descendant(int ancestor_id, int id) {
    int parent_id = fronds[id].parent;
    while (parent_id != INVALID_ID) {
        if (parent_id == ancestor_id) {
            return true;
        }
        parent_id = fronds[parent_id].parent;
    }
    return false;
}
