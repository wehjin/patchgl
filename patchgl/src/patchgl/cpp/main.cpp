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

float random_float_one() {
    return static_cast <float> (rand()) / static_cast <float> (RAND_MAX);
}

int main() {

    srand((unsigned int) std::time(0));

    GlfwDisplay display = GlfwDisplay();

    float letterSize = 1.f;
    float z = -.5f;
    const Frame frame(-letterSize / 2, letterSize / 2, -letterSize / 2, letterSize / 2, z, z);
    const Argb argb(1.f, 1.f, .5f, .2f);
    const Shape shape(L'I');
    ShiftDisplay shiftDisplay = display.withShift(-.5f, .5f);
    shiftDisplay.addPatch(237, frame, shape, argb);
    shiftDisplay.setShift(0.f, -0.4f);

    Command command1;
    int patchId1 = rand();
    command1.mutable_begin_patch()->mutable_color()->set_red(random_float_one());
    command1.mutable_begin_patch()->mutable_color()->set_green(random_float_one());
    command1.mutable_begin_patch()->mutable_color()->set_blue(random_float_one());
    command1.mutable_begin_patch()->mutable_position()->set_left(-.5f);
    command1.mutable_begin_patch()->mutable_position()->set_top(.5f);
    command1.mutable_begin_patch()->mutable_position()->set_right(.5f);
    command1.mutable_begin_patch()->mutable_position()->set_bottom(-.5f);
    command1.mutable_begin_patch()->mutable_position()->set_near(-0.4f);
    command1.mutable_begin_patch()->set_patch_id((unsigned int) patchId1);

    Command command2;
    int patchId2 = rand();
    command2.mutable_begin_patch()->mutable_color()->set_red(0.f);
    command2.mutable_begin_patch()->mutable_color()->set_green(0.f);
    command2.mutable_begin_patch()->mutable_color()->set_blue(1.f);
    command2.mutable_begin_patch()->mutable_position()->set_left(-1.f);
    command2.mutable_begin_patch()->mutable_position()->set_top(1.f);
    command2.mutable_begin_patch()->mutable_position()->set_right(0.f);
    command2.mutable_begin_patch()->mutable_position()->set_bottom(.1f);
    command2.mutable_begin_patch()->mutable_position()->set_near(-.3f);
    command2.mutable_begin_patch()->set_patch_id((unsigned int) patchId2);

    charon charon;
    charon.commands().start_with(command1, command2)
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
