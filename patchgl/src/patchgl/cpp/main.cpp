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
    float isize = .07f;
    patch_position.set_left(-isize / 2);
    patch_position.set_right(isize / 2);
    patch_position.set_bottom(-isize / 2);
    patch_position.set_top(isize / 2);
    patch_position.set_near(-.1f);
    BeginPatch_Color patch_color;
    patch_color.set_red(1.f);
    patch_color.set_green(1.f);
    patch_color.set_blue(1.f);
    patch_color.set_alpha(1.f);
    display.addPatch(237, patch(patch_position, patch_color, L'I'));

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
    display.remove();
    exit(0);

}
