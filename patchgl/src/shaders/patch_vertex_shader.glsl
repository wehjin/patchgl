#version 140

in vec2 position;
uniform mat4 modelview;

void main() {
    gl_Position = modelview * vec4(position, 0.0, 1.0);
}