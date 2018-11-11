#version 330 core

out vec4 Color;

in Vertex_Data {
    vec3 Color;
} IN;

void main()
{
    Color = vec4(IN.Color, 1.0f);
}