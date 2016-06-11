//
// Created by Jeffrey Yu on 6/11/16.
//

#ifndef PATCHGL_SHIFTDISPLAY_H
#define PATCHGL_SHIFTDISPLAY_H

#include <map>
#include "Display.h"

class ShiftDisplay : public Display {

    class PatchSettings {
    public:
        Frame frame;
        Shape shape;
        Argb argb;
        unsigned int id;

        inline PatchSettings(const Frame &frame, const Shape &shape, const Argb &argb, const unsigned int id)
                : frame(frame), shape(shape), argb(argb), id(id) {
        };

        inline bool operator<(const PatchSettings &rhs) const {
            return id < rhs.id;
        }
    };

    class PatchSettingsRemovable : public BooleanRemovable {
        ShiftDisplay *pDisplay;
        const PatchSettings &patchSettings;

    public:
        inline PatchSettingsRemovable(ShiftDisplay *pDisplay, const PatchSettings &patchSettings)
                : pDisplay(pDisplay), patchSettings(patchSettings) {
        }

    protected:
        inline virtual void onRemove() override {
            pDisplay->patch_map[patchSettings]->remove();
            pDisplay->patch_map.erase(patchSettings);
        }
    };

    Display &originalDisplay;
    float horizontal, vertical;
    std::map<PatchSettings, std::shared_ptr<Removable>> patch_map;

public:
    ShiftDisplay(Display &originalDisplay, float horizontalStart, float verticalStart);

    virtual std::shared_ptr<Removable> addPatch(unsigned int id, Frame frame, Shape shape, Argb argb);
};


#endif //PATCHGL_SHIFTDISPLAY_H
