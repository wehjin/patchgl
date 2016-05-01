//
// Created by Jeffrey Yu on 5/1/16.
//

#include "charon.h"

using namespace rxcpp::sources;
using namespace rxcpp::operators;
using namespace rxcpp::util;

observable<patchgl::BeginPatch> charon::begin_patch_requests() {

    patchgl::BeginPatch beginPatch1;
    beginPatch1.mutable_color()->set_red(1.f);
    beginPatch1.mutable_color()->set_green(1.f);
    beginPatch1.mutable_color()->set_blue(1.f);
    beginPatch1.mutable_position()->set_left(-.5f);
    beginPatch1.mutable_position()->set_top(.5f);
    beginPatch1.mutable_position()->set_right(.5f);
    beginPatch1.mutable_position()->set_bottom(-.5f);

    patchgl::BeginPatch beginPatch2;
    beginPatch2.mutable_color()->set_red(1.f);
    beginPatch2.mutable_color()->set_green(1.f);
    beginPatch2.mutable_color()->set_blue(1.f);
    beginPatch2.mutable_position()->set_left(-1.f);
    beginPatch2.mutable_position()->set_top(1.f);
    beginPatch2.mutable_position()->set_right(0.f);
    beginPatch2.mutable_position()->set_bottom(.1f);

    observable<patchgl::BeginPatch> from_io = observable<>::create<patchgl::BeginPatch>(
            [](subscriber<patchgl::BeginPatch> subscriber) {
                patchgl::BeginPatch beginPatch;
                if (beginPatch.ParseFromIstream(&cin)) {
                    subscriber.on_next(beginPatch);
                } else {
                    cerr << "Failed to parse BeginPatch." << endl;
                }
            });
    return from_io.start_with(beginPatch1, beginPatch2);
}
