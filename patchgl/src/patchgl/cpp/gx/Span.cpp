//
// Created by Jeffrey Yu on 6/25/16.
//

#include <GLFW/glfw3.h>
#include "Span.h"

const size_t positionOffset = 0;
const size_t colorOffset = positionOffset + sizeof(PositionSpan);
const size_t textureCoordinateOffset = colorOffset + sizeof(ColorSpan);
const size_t textureUnitOffset = textureCoordinateOffset + sizeof(TextureCoordinateSpan);

const ColorSpan emptyColor = {0.f, 0.f, 0.f};
const PositionSpan emptyPosition = {0.f, 0.f, 0.f};
const TextureCoordinateSpan emptyTextureCoordinate = {0.f, 0.f};
const int emptyTextureUnit = -1;
const VertexSpan emptyVertex = {emptyPosition,
                          emptyColor,
                          emptyTextureCoordinate,
                          emptyTextureUnit};
const PatchSpan emptyPatch = {{emptyVertex, emptyVertex, emptyVertex},
                        {emptyVertex, emptyVertex, emptyVertex}};
