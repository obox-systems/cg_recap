#version 330 core

in vec3 v_world_pos;
in vec3 v_normal;

uniform samplerCube env;
uniform vec3 eye;

out vec4 frag_color;

void main( )
{
    const vec3 LIGHT_DIRECTION = normalize( vec3( 0.0, -1.0, -1.0 ) );
    const vec3 AMBIENT = vec3( 0.3, 0.3, 0.3 );

    vec3 normal = normalize( v_normal );
    vec3 to_eye = normalize( eye - v_world_pos );

    vec3 reflection = reflect( -to_eye, normal );
    // to left-handed
    reflection.z = -reflection.z;

    vec3 illumination = max( dot( -LIGHT_DIRECTION, normal ), 0.0 ) + AMBIENT;
    vec3 color = texture( env, reflection ).rgb;
    frag_color = vec4( color, 1.0 );
}