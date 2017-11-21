#version 410 core

in vec3 vPos;
in vec3 vNormal;
in vec4 vColor;

uniform Transform{
     mat4 model;
     mat4 view;
     mat4 projection;
};

out vec4 worldFragColor;
out vec4 worldNormal;
out vec4 worldPos;

void main()
{
    worldFragColor = vColor;
    worldNormal = view * model * vec4(vNormal, 0);
    worldPos = view * model * vec4(vPos, 1);
    
    gl_Position = projection * worldPos;    
}