#version 410 core

const int MAX_LIGHTS = 8;

in vec4 worldFragColor;

struct Light
{
    vec3 position;
    vec3 direction;
    vec4 diffuse_color;
};

uniform Light lights[MAX_LIGHTS];

uniform lightMeta
{
    int lightCount;
};

out vec4 Target0;

void main()
{
    vec4 color = vec4(vec3(lightCount), 1);
    Target0 = color;
}