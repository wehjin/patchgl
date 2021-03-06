#version 140
uniform mat4 modelview;
in vec3 position;
in vec2 tex_coords;
in vec4 colour;
out vec2 v_tex_coords;
out vec4 v_colour;
void main() {
    gl_Position = modelview * vec4(position, 1.0);
    v_tex_coords = tex_coords;
    v_colour = colour;
}
