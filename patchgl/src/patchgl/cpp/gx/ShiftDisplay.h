//
// Created by Jeffrey Yu on 6/11/16.
//

#ifndef PATCHGL_SHIFTDISPLAY_H
#define PATCHGL_SHIFTDISPLAY_H

#include <map>
#include "Display.h"

class ShiftDisplay : public Display {

    Display &originalDisplay;
    float horizontal, vertical;
    std::map<unsigned int, std::tuple<Frame, Shape, Argb>> patches;

public:
    inline ShiftDisplay(Display &originalDisplay, float horizontalStart, float verticalStart)
            : originalDisplay(originalDisplay), horizontal(horizontalStart), vertical(verticalStart) {
    }

    inline virtual void addPatch(unsigned int patchId, Frame frame, Shape shape, Argb argb) override {
        patches[patchId] = std::make_tuple(frame, shape, argb);
        originalDisplay.addPatch(patchId, frame.withShift(horizontal, vertical), shape, argb);
    }

    inline virtual void removePatch(unsigned int patchId) override {
        patches.erase(patchId);
        originalDisplay.removePatch(patchId);
    }

    void setShift(float horizontal, float vertical) {
        this->horizontal = horizontal;
        this->vertical = vertical;
        for (auto entry : patches) {
            unsigned int patchId = entry.first;
            Frame &frame = std::get<0>(entry.second);
            Shape &shape = std::get<1>(entry.second);
            Argb &argb = std::get<2>(entry.second);
            originalDisplay.addPatch(patchId, frame.withShift(horizontal, vertical), shape, argb);
        }
    }
};


#endif //PATCHGL_SHIFTDISPLAY_H
