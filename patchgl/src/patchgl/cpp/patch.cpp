//
// Created by Jeffrey Yu on 4/30/16.
//

#include "patch.h"

patch::patch(const patchgl::BeginPatch_Position &position, const patchgl::BeginPatch_Color &color)
        : left(position.left()),
          bottom(position.bottom()),
          right(position.right()),
          top(position.top()),
          near(position.near()),
          red(color.red()),
          green(color.green()),
          blue(color.blue()),
          alpha(color.alpha()) { }


