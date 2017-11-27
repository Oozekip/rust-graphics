#version 410 core

const int MAX_LIGHTS = 8;

in vec4 worldNormal;
in vec4 worldPos;
in vec2 UV;

uniform sampler2D diffuseTexture;
uniform sampler2D specularTexture;

struct Light
{
    vec4 diffuse;
    vec4 ambient;
    vec4 specular;
    vec4 position;
    vec4 direction;
    int type;   // 0 directional, 1 point, 2 spot
    float spotlightOuter;
    float spotlightInner;
    float spotlightFalloff;
};

layout(std140)
uniform materialData{
    vec4 m_diffuse;
    vec4 m_ambient;
    vec4 m_specular;
    float m_specularPower;
    int m_useDiffuseTexture;
    int m_useSpecularTexture;
};

layout(std140)
uniform lightMeta
{
    int lightCount;
};

layout(std140)
uniform lightData
{
    Light lights[MAX_LIGHTS];
};

out vec4 Target0;

vec4 computeDiffuse(in vec4 N, in vec4 L, in vec4 iD, in vec4 kD)
{
    return iD * kD * clamp(dot(L, N), 0, 1);
}

vec4 computeAmbient(in vec4 iA, in vec4 kA)
{
    return iA * kA;
}

vec4 computeSpecular(in vec4 N, in vec4 L, in vec4 V, in vec4 iS, in vec4 kS, float power)
{
    vec4 R = normalize(reflect(L, normalize(N)));

    // pow(<= 0) is undefined in GLSL
    float finalPower = (power != 0) ? pow(clamp(dot(R, V), 0, 1), power): 1;
    
    return iS * kS * finalPower;
}

vec4 computeLighting(in vec4 worldNorm, in vec4 worldPos){
    vec4 litColor = vec4(0);

    for(int i  = 0; i < lightCount; i++){
        Light light = lights[i];

        vec4 L;

        switch(light.type)
        {
            case 1:
                L = normalize(light.position - worldPos);
                break;
            default:
                L = normalize(-light.direction);
                break;
        }

        vec4 diffColor = (m_useDiffuseTexture != 0) ? texture2D(diffuseTexture, UV): m_diffuse;
        vec4 specColor = (m_useSpecularTexture != 0) ? texture2D(specularTexture, UV): m_specular;
        float specPower = (m_useSpecularTexture != 0) ? specColor.r * 255: m_specularPower;

        vec4 ambient = computeAmbient(light.ambient, m_ambient); 
        vec4 diff = computeDiffuse(worldNorm, L, light.diffuse, diffColor);
        vec4 spec = computeSpecular(worldNorm, L, vec4(0, 0, -1, 0), light.specular, specColor, specPower);

        float dv = length(-vec4(worldPos.xyz, 0));

        float attenuation;
        float spotlight;

        switch(light.type)
        {
            case 0:
                // No attenuation for directional lights
                attenuation = 1;
                break;
            default:
                attenuation = min(1 / pow(dv, 2), 1);
                break;
        }

        switch(light.type)
        {
            case 2: // Spotlight effect
            {
                float cosAlpha = max(dot(L, normalize(light.position - worldPos)), 0);
                float cosPhi = cos(light.spotlightOuter);
                float cosTheta = cos(light.spotlightInner);

                spotlight = pow((cosAlpha - cosPhi) / (cosTheta - cosPhi), light.spotlightFalloff);
            }break;
            
            default:
                spotlight = 1;
                break;
        }
        litColor += ambient +  spotlight * attenuation * (diff + spec);
    }

    return vec4(litColor.rgb, 1);
}

void main()
{
    vec4 diffColor = computeLighting(worldNormal, worldPos);
    Target0 = vec4(diffColor.rgb, 1);
}
