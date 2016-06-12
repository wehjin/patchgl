//
// Created by Jeffrey Yu on 6/11/16.
//

#ifndef PATCHGL_SHAPE_H
#define PATCHGL_SHAPE_H

class Shape {
public:
    wchar_t letter;

    inline Shape(const wchar_t letter = L'\u2588')
            : letter(letter) {
    }

    inline Shape(const Shape &other)
            : letter(other.letter) { }

    inline Shape &operator=(const Shape &other) {
        letter = other.letter;
        return *this;
    }
};


#endif //PATCHGL_SHAPE_H
