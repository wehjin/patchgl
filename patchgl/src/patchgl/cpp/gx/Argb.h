//
// Created by Jeffrey Yu on 6/11/16.
//

#ifndef PATCHGL_ARGB_H
#define PATCHGL_ARGB_H


class Argb {
public:
    Argb(float alpha, float red, float green, float blue);

    float red, green, blue, alpha;
};

inline Argb::Argb(float alpha, float red, float green, float blue)
        : alpha(alpha), red(red), green(green), blue(blue) {
}

#endif //PATCHGL_ARGB_H
