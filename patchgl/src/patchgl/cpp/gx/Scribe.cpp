//
// Created by Jeffrey Yu on 6/18/16.
//

#include <ft2build.h>
#include FT_FREETYPE_H
#include <exception>
#include <stdexcept>
#include <string>
#include <iostream>
#include <algorithm>

#include "Scribe.h"
#include "../font.h"

FT_Library library;
FT_Face face;
bool initialized = false;

std::string getErrorMessage(const char *tag, int error) {
    return std::string(tag) + std::string(":") + std::to_string(error);
}

Scribe::Scribe() {

    if (!initialized) {
        int error = FT_Init_FreeType(&library);
        if (error) {
            throw std::runtime_error(getErrorMessage("FT_Init_FreeType", error));
        }
        error = FT_New_Memory_Face(library, Font_ttf, Font_ttf_len, 0, &face);
        if (error) {
            throw std::runtime_error(getErrorMessage("FT_New_Memory_Face", error));
        }
        error = FT_Set_Char_Size(
                face,    /* handle to face object           */
                0,       /* char_width in 1/64th of points  */
                16 * 64,   /* char_height in 1/64th of points */
                150,     /* horizontal device resolution    */
                0);   /* vertical device resolution      */
        if (error) {
            throw std::runtime_error(getErrorMessage("FT_Set_Char_Size", error));
        }

        atlasWidth = 0;
        atlasHeight = 0;
        for (unsigned long i = 32; i < 128; i++) {
            error = FT_Load_Char(face, i, FT_LOAD_RENDER);
            if (error) {
                throw std::runtime_error(getErrorMessage("FT_Load_Char", error));
            }
            FT_GlyphSlot glyphSlot = face->glyph;
            printBitmap(glyphSlot->bitmap);
            characterInfoArray[i].advanceX = glyphSlot->advance.x >> 6;
            characterInfoArray[i].advanceY = glyphSlot->advance.y >> 6;
            characterInfoArray[i].bitmapWidth = glyphSlot->bitmap.width;
            characterInfoArray[i].bitmapHeight = glyphSlot->bitmap.rows;
            characterInfoArray[i].bitmapLeft = glyphSlot->bitmap_left;
            characterInfoArray[i].bitmapTop = glyphSlot->bitmap_top;
            characterInfoArray[i].atlasXInt = atlasWidth;
            atlasWidth += glyphSlot->bitmap.width;
            atlasHeight = std::max(atlasHeight, glyphSlot->bitmap.rows);
            atlasTop = std::max(atlasTop, glyphSlot->bitmap_top);
        }
        for (unsigned long i = 32; i < 128; i++) {
            characterInfoArray[i].atlasX = ((float) characterInfoArray[i].atlasXInt) / atlasWidth;
        }
        std::cout << "Atlas top: " << atlasTop << std::endl;
        std::cout << "Atlas height: " << atlasHeight << std::endl;
        initialized = true;
    }
}


void Scribe::setIndex(unsigned long index) {
    FT_Error error = FT_Load_Char(face, index, FT_LOAD_RENDER);
    if (error) {
        throw std::runtime_error(getErrorMessage("FT_Load_Char", error));
    }
}

int Scribe::getWidth() const {
    return face->glyph->bitmap.width;
}


int Scribe::getHeight() const {
    return face->glyph->bitmap.rows;
}

void *Scribe::getImage() const {
    return face->glyph->bitmap.buffer;
}

void Scribe::printBitmap(const FT_Bitmap &bitmap) const {
    std::__1::cout << "Bitmap width: " << bitmap.width << std::__1::endl;
    std::__1::cout << "Bitmap rows: " << bitmap.rows << std::__1::endl;
    for (int i = 0; i < bitmap.rows; i++) {
        unsigned int rowStart = i * bitmap.width;
        for (int j = 0; j < bitmap.width; j++) {
            std::__1::cout << ((int) bitmap.buffer[rowStart + j]) << " ";
        }
        std::__1::cout << std::__1::endl;
    }
}





