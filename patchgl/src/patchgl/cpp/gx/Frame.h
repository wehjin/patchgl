//
// Created by Jeffrey Yu on 6/11/16.
//

#ifndef PATCHGL_FRAME_H
#define PATCHGL_FRAME_H


class Frame {
public:

    float left, right, top, bottom, near, far;

    inline Frame(const float left = 0.0f, const float right = 0.0f, const float bottom = 0.0f, const float top = 0.0f,
                 const float near = 0.0f, const float far = 0.0f)
            : left(left), right(right), bottom(bottom), top(top), near(near), far(far) {
    }

    inline Frame(const Frame &other)
            : left(other.left),
              right(other.right),
              bottom(other.bottom),
              top(other.top),
              near(other.near),
              far(other.far) { }

    inline Frame &operator=(const Frame &other) {
        left = other.left;
        right = other.right;
        bottom = other.bottom;
        top = other.top;
        near = other.near;
        far = other.far;
        return *this;
    }

    inline Frame withShift(float horizontal, float vertical) {
        return Frame(left + horizontal, right + horizontal, bottom + vertical, top + vertical, near, far);
    }
};

#endif //PATCHGL_FRAME_H
