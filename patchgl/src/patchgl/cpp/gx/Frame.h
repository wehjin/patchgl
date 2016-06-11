//
// Created by Jeffrey Yu on 6/11/16.
//

#ifndef PATCHGL_FRAME_H
#define PATCHGL_FRAME_H


class Frame {
public:
    Frame(float left, float right, float bottom, float top, float near, float far);

    float left, right, top, bottom, near, far;

    Frame withShift(float horizontal, float vertical);
};

inline Frame::Frame(float left, float right, float bottom, float top, float near, float far)
        : left(left), right(right), bottom(bottom), top(top), near(near), far(far) {
}

inline Frame Frame::withShift(float horizontal, float vertical) {
    return Frame(left + horizontal, right + horizontal, bottom + vertical, top + vertical, near, far);
}

#endif //PATCHGL_FRAME_H
