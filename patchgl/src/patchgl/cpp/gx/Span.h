//
// Created by Jeffrey Yu on 6/25/16.
//

#ifndef PATCHGL_SPAN_H
#define PATCHGL_SPAN_H

typedef struct {
    GLfloat x, y, z;
} PositionSpan;

typedef struct {
    GLfloat r, g, b;
} ColorSpan;

typedef struct {
    GLfloat s, t;
} TextureCoordinateSpan;

#pragma pack(1)
typedef struct {
    PositionSpan position;
    ColorSpan color;
    TextureCoordinateSpan textureCoordinate;
    GLint textureUnit;
} VertexSpan;

#pragma pack(1)
typedef struct {
    VertexSpan bl, br, tr;
} BottomRightTriangle;

#pragma pack(1)
typedef struct {
    VertexSpan tr, tl, bl;
} TopLeftTriangle;

#pragma pack(1)
typedef struct {
    BottomRightTriangle bottomRight;
    TopLeftTriangle topLeft;
} PatchSpan;

extern const size_t positionOffset;
extern const size_t colorOffset;
extern const size_t textureCoordinateOffset;
extern const size_t textureUnitOffset;

extern const ColorSpan emptyColor;
extern const PositionSpan emptyPosition;
extern const TextureCoordinateSpan emptyTextureCoordinate;
extern const int emptyTextureUnit;
extern const VertexSpan emptyVertex;
extern const PatchSpan emptyPatch;

#endif //PATCHGL_SPAN_H
