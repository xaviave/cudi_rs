// precision highp float;

// in vec2 vert;
// out vec4 fragColor;

// void main() {
//     fragColor = vec4(vert, 0.5, 1.0);
// }
out vec4 FragColor;
  
in vec3 ourColor;
in vec2 TexCoord;

uniform sampler2D ourTexture;

void main()
{
    FragColor = texture(ourTexture, TexCoord);
}