//
// Created by Jeffrey Yu on 6/11/16.
//

#ifndef PATCHGL_SHAPE_H
#define PATCHGL_SHAPE_H


class Shape {
public:
    Shape(wchar_t letter);

    wchar_t letter;
};

inline Shape::Shape(wchar_t letter)
        : letter(letter) {
}


#endif //PATCHGL_SHAPE_H
