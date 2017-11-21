#version 410 core

const int MAX_LIGHTS = 8;

in vec4 worldFragColor;
in vec4 worldNormal;
in vec4 worldPos;

struct Light
{
    vec3 direction;
    vec4 diffuseColor;
};

uniform Light lights[MAX_LIGHTS];

layout (std140)
uniform lightMeta
{
    int lightCount;
};

out vec4 Target0;

vec4 computeLighting(in vec4 worldNorm, in vec4 worldPos){
    vec4 litColor = vec4(0);

    for(int i  = 0; i < lightCount; i++){
        Light light = lights[i];

        vec4 currColor = light.diffuseColor * worldFragColor * clamp(dot(vec4(-light.direction, 0), worldNorm), 0, 1);

        litColor += currColor;
    }

    return vec4(litColor.rgb, worldFragColor.a);
}

void main()
{
    vec4 diffColor = computeLighting(worldNormal, worldPos);
    Target0 = vec4(diffColor.rgb, 1);
}