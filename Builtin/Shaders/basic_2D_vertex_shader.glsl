#version 330 core

layout (location = 0) in vec3 aPos;

uniform mat4 punk_model;
uniform mat4 punk_projection;

void main()
{
    gl_Position = punk_projection * punk_model * vec4(aPos, 1.0f);
}
