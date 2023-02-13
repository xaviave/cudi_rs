#version 330

const vec2 positions[3] = vec2[3](
    vec2( 0.0f,  1.0f),
    vec2(-1.0f, -1.0f),
    vec2( 1.0f, -1.0f)
);

const vec3 colors[3] = vec3[3](
    vec3(1, 0, 0),
    vec3(0, 1, 0),
    vec3(0, 0, 1)
);

out vec4 vColor;

uniform mat4 uMMatrix;
uniform mat4 uVMatrix;
uniform mat4 uPMatrix;

void main() {
    gl_Position = uPMatrix * uVMatrix * uMMatrix *vec4(positions[gl_VertexID % 3], 0.0, 1.0);
    vColor = vec4(colors[gl_VertexID % 3], 1.0);
}

