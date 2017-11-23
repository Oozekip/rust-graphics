#version 410 core

in vec3 vPos;
in vec3 vNormal;

layout(std140)
uniform Transform{
     mat4 model;
     mat4 view;
     mat4 projection;
};

out vec4 worldNormal;
out vec4 worldPos;

void main()
{
    worldNormal = view * model * vec4(vNormal, 0);
    worldPos = view * model * vec4(vPos, 1);
    
    gl_Position = projection * worldPos;    
}