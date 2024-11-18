#version 330 core

uniform mat4 clip2world;

out vec3 direction;

void main()
{
    const vec2 POSITIONS[] = vec2[]( vec2( -1.0, -1.0 ), vec2( 3.0, -1.0 ), vec2( -1.0, 3.0 ) );
    vec2 position = POSITIONS[ gl_VertexID ];
    direction = ( clip2world * vec4( position, 0.999, 1.0 ) ).xyz;
    gl_Position = vec4( position, 0.0, 1.0 );
}