precision highp float;

in vec2 vert;
out vec4 fragColor;

void main() {
    fragColor = vec4(vert, 0.5, 1.0);
}