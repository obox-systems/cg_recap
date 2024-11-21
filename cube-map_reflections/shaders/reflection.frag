#version 330 core

in vec3 v_world_pos;
in vec3 v_normal;

uniform samplerCube env;
uniform vec3 eye;

out vec4 frag_color;

void main( )
{
    vec3 normal = normalize( v_normal );
    vec3 to_eye = normalize( eye - v_world_pos );

    vec3 reflection = reflect( -to_eye, normal );
    // to left-handed
    reflection.z = -reflection.z;

    vec3 color = texture( env, reflection ).rgb;
    frag_color = vec4( color, 1.0 );
}