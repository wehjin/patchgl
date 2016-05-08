//
// Created by Jeffrey Yu on 5/1/16.
//

#include "charon.h"

using namespace rxcpp::sources;
using namespace rxcpp::operators;
using namespace rxcpp::util;
using namespace patchgl;

observable<Command> charon::commands() {

    Command command1;
    command1.mutable_begin_patch()->set_patch_id((unsigned int) rand());
    command1.mutable_begin_patch()->mutable_color()->set_red(0.f);
    command1.mutable_begin_patch()->mutable_color()->set_green(1.f);
    command1.mutable_begin_patch()->mutable_color()->set_blue(0.f);
    command1.mutable_begin_patch()->mutable_position()->set_left(-.5f);
    command1.mutable_begin_patch()->mutable_position()->set_top(.5f);
    command1.mutable_begin_patch()->mutable_position()->set_right(.5f);
    command1.mutable_begin_patch()->mutable_position()->set_bottom(-.5f);
    command1.mutable_begin_patch()->mutable_position()->set_near(-0.4f);

    Command command2;
    command1.mutable_begin_patch()->set_patch_id((unsigned int) rand());
    command2.mutable_begin_patch()->mutable_color()->set_red(0.f);
    command2.mutable_begin_patch()->mutable_color()->set_green(0.f);
    command2.mutable_begin_patch()->mutable_color()->set_blue(1.f);
    command2.mutable_begin_patch()->mutable_position()->set_left(-1.f);
    command2.mutable_begin_patch()->mutable_position()->set_top(1.f);
    command2.mutable_begin_patch()->mutable_position()->set_right(0.f);
    command2.mutable_begin_patch()->mutable_position()->set_bottom(.1f);
    command2.mutable_begin_patch()->mutable_position()->set_near(-.5f);

    return observable<void, void>::create<Command>([](subscriber<Command> subscriber) {
        for (; ;) {
            char byte;
            cin >> setw(1) >> byte;
            if (cin.eof()) {
                cerr << "Out of bytes" << endl;
                break;
            }

            int size = byte;
            cerr << "Size: " << size << endl;

            char buffer[size];
            cin.read(buffer, size);
            if (cin.fail()) {
                cerr << "Error reading buffer" << endl;
                break;
            }

            patchgl::Command command;
            bool parsed = command.ParseFromArray(buffer, size);
            if (parsed) {
                subscriber.on_next(command);
            } else {
                cerr << "Failed to parse BeginPatch." << endl;
            }
        }
    }).start_with(command1, command2);
}

