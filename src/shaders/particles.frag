#version 400

uniform sampler2D circle_texture;
uniform vec2 resolution; // delete
uniform float circle_diameter; // delete
uniform float time;

in vec2 g_uv;

out vec4 f_color;

vec3 color = vec3(0.17968, 0.79687, 0.44140); // green

// https://github.com/mattdesl/lwjgl-basics/wiki/ShaderLesson3
// https://en.wikibooks.org/wiki/OpenGL_Programming/Depth_of_Field
// + motion blur
// https://www.shadertoy.com/view/XdXXz4
// https://www.shadertoy.com/view/MdSGDm
// + shadows...
void    main() {
    f_color = texture(circle_texture, g_uv) * vec4(color, 1.0);
    if (f_color.a < 1.0) {
        discard;
    }

    // vec4 tex_color = texture(circle_texture, g_uv);
    // f_color = tex_color;

    // f_color = texture(circle_texture, g_uv) * vec4(color, 1.0);

    // f_color = circle(g_uv) * vec4(color, 1.0);
}
