# Environment mapping

OpenGl support for environment mapping is cube map textures.

The cube map texture has 6 faces that are aligned in **left-handed** coordinate system.
To sample texture use 3-component vector that represents direction from center of a cube into a face.
Assure that vector is in left-handed coordinate system or you will end up sampling opposite site of a cube.
For this probably just invert z component of the vector.

There's `gl::TEXTURE_CUBE_MAP_SEAMLESS` flag that will enable texture filtering within several faces at a time.
This will hide edges where textures join each other, though it is not free.

## Mapping shader

Fast and easy way to draw cube map as backgound image is to draw a large triangle that covers the whole screen.
Its coordinates may easily be obtained in clip-space as `[-1; -1]`, `[3; -1]`, `[-1; 3]`. This coordinates should be
converted with **inverse view-projection** matrix into world-space coordinates that will be used as texture coordinates for cube map.
Most likely the inverse view-projection matrix should not have any translation, otherwise it will affect obtainig texture coordinates,
so **don't forget** to ignore translation component of a view matrix when calculating inverse-view matrix.
Provide obtained coordinates into fragment shader:

``` glsl
v_texcoord = ( clip2world * vec4( position, 1.0, 1.0 ) ).xyz;
// convertion from right-handed to left-handed
v_texcoord.z = -v_texcoord.z;
```

Set `z` component for `gl_Position` output as 0.999 for the fragment to be at the most farthest part of the scene:

``` glsl
gl_Position = vec4( position, 0.999, 1.0 );
```
