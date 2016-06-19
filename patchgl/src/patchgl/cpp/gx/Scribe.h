//
// Created by Jeffrey Yu on 6/18/16.
//

#ifndef PATCHGL_SCRIBE_H
#define PATCHGL_SCRIBE_H

class Scribe {
private:
    unsigned int atlasWidth = 0;
    unsigned int atlasHeight = 0;

public:

    struct character_info {
        float advanceX;
        float advanceY;
        float bitmapWidth;
        float bitmapHeight;
        float bitmapLeft;
        float bitmapTop;
        unsigned int atlasXInt;
        float atlasX;
        float atlasEndX;
    } characterInfoArray[128];
    int atlasTop = 0;
    unsigned int maxBitmapWidth = 0;
    int gap = 1;

    Scribe();

    inline int getAtlasWidth() const {
        return atlasWidth;
    }

    inline int getAtlasHeight() const {
        return atlasHeight;
    }

    void setIndex(unsigned long index);

    int getWidth() const;

    int getHeight() const;

    void *getImage() const;

    void printBitmap(const FT_Bitmap &bitmap) const;
};


#endif //PATCHGL_SCRIBE_H
