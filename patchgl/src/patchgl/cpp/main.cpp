#define GLFW_INCLUDE_GLU

#include <stdlib.h>
#include <map>
#include <cstdlib>
#include <ctime>
#include <set>
#include <fstream>
#include <iostream>
#include <glm/glm.hpp>

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

    float letterSize = .8f;
    float z = .02f;
    const Frame frame(-letterSize / 2, letterSize / 2, -letterSize / 2, letterSize / 2, z, z);
    const Argb argb(1.f, 1.f, .5f, .2f);
    const Shape shape(L'W');
    ShiftDisplay shiftDisplay = display.withShift(-.5f, .5f);
    shiftDisplay.addPatch(237, frame, shape, argb);
    shiftDisplay.setShift(0.f, -0.0f);

    Command command1;
    int patchId1 = rand();
    command1.mutable_begin_patch()->mutable_color()->set_red(random_float_one());
    command1.mutable_begin_patch()->mutable_color()->set_green(random_float_one());
    command1.mutable_begin_patch()->mutable_color()->set_blue(random_float_one());
    command1.mutable_begin_patch()->mutable_position()->set_left(-.5f);
    command1.mutable_begin_patch()->mutable_position()->set_top(.5f);
    command1.mutable_begin_patch()->mutable_position()->set_right(.5f);
    command1.mutable_begin_patch()->mutable_position()->set_bottom(-.5f);
    command1.mutable_begin_patch()->mutable_position()->set_near(.015f);
    command1.mutable_begin_patch()->set_patch_id((unsigned int) patchId1);
    command1.mutable_begin_patch()->set_shape("");

    Command command2;
    int patchId2 = rand();
    command2.mutable_begin_patch()->mutable_color()->set_red(0.f);
    command2.mutable_begin_patch()->mutable_color()->set_green(random_float_one());
    command2.mutable_begin_patch()->mutable_color()->set_blue(1.f);
    command2.mutable_begin_patch()->mutable_position()->set_left(-1.5f);
    command2.mutable_begin_patch()->mutable_position()->set_bottom(-1.f);
    command2.mutable_begin_patch()->mutable_position()->set_top(1.f);
    command2.mutable_begin_patch()->mutable_position()->set_right(1.5f);
    command2.mutable_begin_patch()->mutable_position()->set_near(.01f);
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
                    const string &utf8 = beginPatch.shape();
                    const std::wstring utf16(utf8.begin(), utf8.end());
                    display.addPatch(beginPatch.patch_id(),
                                     patch(position, color, utf16.empty() ? L'\u2588' : utf16[0]));
                } else if (command.has_end_patch()) {
                    display.removePatch(command.end_patch().patch_id());
                }
            });

    display.awaitClose();
    exit(0);

}
