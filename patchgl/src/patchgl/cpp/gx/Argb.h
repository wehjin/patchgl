//
// Created by Jeffrey Yu on 6/11/16.
//

#ifndef PATCHGL_ARGB_H
#define PATCHGL_ARGB_H


class Argb {
public:
    float red, green, blue, alpha;

    inline Argb(const float alpha = 1.f, const float red = 1.f, const float green = 1.f, const float blue = 1.f)
            : alpha(alpha), red(red), green(green), blue(blue) {
    };

    inline Argb(const Argb &other)
            : red(other.red), green(other.green), blue(other.blue), alpha(other.alpha) { }

    inline Argb &operator=(const Argb &other) {
        red = other.red;
        green = other.green;
        blue = other.blue;
        alpha = other.alpha;
        return *this;
    }
};

#endif //PATCHGL_ARGB_H
