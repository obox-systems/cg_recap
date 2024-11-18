#version 330 core

in vec3 v_texcoord;

uniform samplerCube env;

out vec4 frag_color;

void main()
{
    frag_color = texture( env, v_texcoord );
}