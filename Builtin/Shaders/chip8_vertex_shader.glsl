#version 330 core

layout (location = 0) in vec3 aPos;

uniform mat4 chip_transform;

void main()
{
    gl_Position = chip_transform * vec4(aPos, 1.0f);
}
