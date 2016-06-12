#define GLFW_INCLUDE_GLU

#include <stdlib.h>
#include <map>
#include <cstdlib>
#include <ctime>
#include <set>
#include <fstream>
#include <iostream>
#include "patch.h"

#include "rxcpp/rx.hpp"
#include "charon.h"
#include "gx/GlfwDisplay.h"

using namespace rxcpp;
using namespace rxcpp::sources;
using namespace rxcpp::operators;
using namespace rxcpp::util;
using namespace std;


int main() {

    srand((unsigned int) std::time(0));

    GlfwDisplay display = GlfwDisplay();

    BeginPatch_Position patch_position;
    float letterSize = .5f;
    patch_position.set_left(-letterSize / 2);
    patch_position.set_right(letterSize / 2);
    patch_position.set_bottom(-letterSize / 2);
    patch_position.set_top(letterSize / 2);
    patch_position.set_near(-.1f);
    const Frame frame(patch_position.left(), patch_position.right(), patch_position.bottom(), patch_position.top(),
                      patch_position.near(), patch_position.near());
    BeginPatch_Color patch_color;
    patch_color.set_red(1.f);
    patch_color.set_green(1.f);
    patch_color.set_blue(1.f);
    patch_color.set_alpha(1.f);
    const Argb argb(patch_color.alpha(), patch_color.red(), patch_color.green(), patch_color.blue());
    const Shape shape(L'I');
    ShiftDisplay shiftDisplay = display.withShift(-.5f, .5f);
    shiftDisplay.addPatch(237, frame, shape, argb);
    shiftDisplay.setShift(0.f, -.5f);

    charon charon;
    charon.commands()
            .subscribe_on(observe_on_new_thread())
            .observe_on(display.scheduler)
            .subscribe([&](Command command) {
                if (command.has_close()) {
                    display.close();
                } else if (command.has_begin_patch()) {
                    const BeginPatch &beginPatch = command.begin_patch();
                    const BeginPatch_Position &position = beginPatch.position();
                    const BeginPatch_Color &color = beginPatch.color();
                    display.addPatch(beginPatch.patch_id(), patch(position, color));
                } else if (command.has_end_patch()) {
                    display.removePatch(command.end_patch().patch_id());
                }
            });

    display.awaitClose();
    exit(0);

}
