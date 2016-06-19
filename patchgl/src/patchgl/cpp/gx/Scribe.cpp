//
// Created by Jeffrey Yu on 6/18/16.
//

#include <ft2build.h>
#include FT_FREETYPE_H
#include <exception>
#include <stdexcept>
#include <string>
#include <iostream>

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
        FT_UInt glyph_index = FT_Get_Char_Index(face, 'M');
        error = FT_Load_Glyph(face, glyph_index, FT_LOAD_DEFAULT);
        if (error) {
            throw std::runtime_error(getErrorMessage("FT_Load_Glyph", error));
        }
        error = FT_Render_Glyph(face->glyph, FT_RENDER_MODE_NORMAL);
        if (error) {
            throw std::runtime_error(getErrorMessage("FT_Render_Glyph", error));
        }

        FT_GlyphSlot slot = face->glyph;
        FT_Bitmap &bitmap = slot->bitmap;
        std::cout << "Bitmap width: " << bitmap.width << std::endl;
        std::cout << "Bitmap rows: " << bitmap.rows << std::endl;
        for (int i = 0; i < bitmap.rows; i++) {
            unsigned int rowStart = i * bitmap.width;
            for (int j = 0; j < bitmap.width; j++) {
                std::cout << ((int) bitmap.buffer[rowStart + j]) << " ";
            }
            std::cout << std::endl;
        }
        initialized = true;
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



