#version 330 core

uniform mat4 clip2world;

out vec3 v_texcoord;

void main()
{
    const vec2 POSITIONS[] = vec2[]
    ( 
        vec2( -1.0, -1.0 ),
        vec2(  3.0, -1.0 ),
        vec2( -1.0,  3.0 )
    );
    vec2 position = POSITIONS[ gl_VertexID ];
    v_texcoord = ( clip2world * vec4( position, 1.0, 1.0 ) ).xyz;
    v_texcoord.z = -v_texcoord.z;
    gl_Position = vec4( position, 0.999, 1.0 );
}