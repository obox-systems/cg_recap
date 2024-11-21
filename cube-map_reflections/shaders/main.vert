#version 330 core

layout ( location = 0 ) in vec3 position;
layout ( location = 1 ) in vec3 normal;

uniform mat4 mvp;
uniform mat4 model;

out vec3 v_world_pos;
out vec3 v_normal;

void main( )
{
    v_world_pos = ( model * vec4( position, 1.0 ) ).xyz;
    v_normal = mat3( model ) * normal;
    gl_Position = mvp * vec4( position, 1.0 );
}