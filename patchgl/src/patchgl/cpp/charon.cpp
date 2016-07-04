//
// Created by Jeffrey Yu on 5/1/16.
//

#include "charon.h"

using namespace rxcpp::sources;
using namespace rxcpp::operators;
using namespace rxcpp::util;
using namespace std;
using namespace patchgl;

observable<Command> charon::commands() {

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
    });
}

