const vec2 verts[3] = vec2[3](
    vec2(0.5f, 1.0f),
    vec2(0.0f, 0.0f),
    vec2(1.0f, 0.0f)
);

layout (location = 0) in vec3 aPos;

out vec2 vert;
void main() {
    vert = verts[gl_VertexID];
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
