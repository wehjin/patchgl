//
// Created by Jeffrey Yu on 4/30/16.
//

#include "patch.h"

patch::patch(const patchgl::BeginPatch_Position &position, const patchgl::BeginPatch_Color &color, const wchar_t shape)
        : left(position.left()),
          bottom(position.bottom()),
          right(position.right()),
          top(position.top()),
          near(position.near()),
          red(color.red()),
          green(color.green()),
          blue(color.blue()),
          alpha(color.alpha()),
          shape(shape) { }

patch::patch(const Frame &frame, const Argb &argb, const Shape &shape)
        : left(frame.left),
          bottom(frame.bottom),
          right(frame.right),
          top(frame.top),
          near(frame.near),
          red(argb.red),
          green(argb.green),
          blue(argb.blue),
          alpha(argb.alpha),
          shape(shape.letter) {
}
