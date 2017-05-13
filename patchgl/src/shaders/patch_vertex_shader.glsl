#version 140

in vec3 position;
uniform mat4 modelview;

void main() {
    gl_Position = modelview * vec4(position, 1.0);
}