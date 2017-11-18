#version 410 core

in vec3 vPos;
in vec4 vColor;

uniform Transform{
     mat4 model;
     mat4 view;
     mat4 projection;
};

out vec4 worldFragColor;

void main()
{
    worldFragColor = vColor;

    gl_Position = projection * view * model * vec4(vPos, 1);    
}