#version 330 core
in vec3 ourColor;
in vec2 TexCoord;
flat in int TexUnit;

out vec4 color;

uniform sampler2D ourTexture;

void main()
{
  if (TexUnit < 0) {
    color = vec4(ourColor, 1.f);
  } else {
    vec4 textureValue = texture(ourTexture, TexCoord);
    if (textureValue.r < .5f) {
      discard;
    }
    color = vec4(ourColor, 1.f);
  }
}
