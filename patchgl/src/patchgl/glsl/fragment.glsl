#version 330 core
in vec3 ourColor;
in vec2 TexCoord;

out vec4 color;

uniform sampler2D ourTexture;

void main()
{
  vec4 textureValue = texture(ourTexture, TexCoord);
  if (textureValue.r < .01f) {
    discard;
  }
  color = vec4(ourColor, textureValue.r);
}
