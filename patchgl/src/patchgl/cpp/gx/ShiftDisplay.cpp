//
// Created by Jeffrey Yu on 6/11/16.
//

#include "ShiftDisplay.h"


ShiftDisplay::ShiftDisplay(Display &originalDisplay, float horizontalStart, float verticalStart)
        : originalDisplay(originalDisplay), horizontal(horizontalStart), vertical(verticalStart) {
}

std::shared_ptr<Removable> ShiftDisplay::addPatch(unsigned int id, Frame frame, Shape shape, Argb argb) {
    const PatchSettings &patchSettings = PatchSettings(frame, shape, argb, id);
    const Frame &shiftedFrame = frame.withShift(horizontal, vertical);
    patch_map[patchSettings] = originalDisplay.addPatch(id, shiftedFrame, shape, argb);
    std::shared_ptr<Removable> removable(new PatchSettingsRemovable(this, patchSettings));
    return removable;
}
