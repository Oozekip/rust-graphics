#version 410 core

const int MAX_LIGHTS = 8;

in vec4 worldFragColor;

struct Light
{
    vec4 diffuseColor;
    vec4 position;
    vec4 direction;
};

layout (std140)
uniform lightData
{
    Light lights[MAX_LIGHTS];
};

// layout (std140)
uniform lightMeta
{
    int lightCount;
};


out vec4 Target0;


void main()
{
     vec4 color = lights[0].position + lights[0].direction - lights[0].diffuseColor;
    Target0 = color;
}