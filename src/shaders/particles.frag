#version 400

uniform sampler2D circle_texture;
uniform vec2 resolution; // delete
uniform float circle_diameter; // delete
uniform float time;

in vec2 g_uv;

out vec4 f_color;

vec3 color = vec3(0.17968, 0.79687, 0.44140); // green
// vec3 color = vec3(0.796875, 0.1796875, 0.2265625); // red
// vec3 color = vec3(1.0, 1.0, 1.0); // white

vec4    circle(vec2 uv) {
    uv *= 2.0; uv -= 1.0; // put in range [-1.0, 1.0]
    vec2 pos = vec2(uv.x, uv.y);
    float dist = length(pos);
    float circle_radius = 1.0;

    if (dist > circle_radius) {
        discard;
    }
    return vec4(1.0, 1.0, 1.0, 1.0);
}

void    main() {
    // https://github.com/mattdesl/lwjgl-basics/wiki/ShaderLesson3
    // https://en.wikibooks.org/wiki/OpenGL_Programming/Depth_of_Field
    // + motion blur
    // https://www.shadertoy.com/view/XdXXz4
    // https://www.shadertoy.com/view/MdSGDm
    // + shadows...

    // vec4 tex_color = texture(circle_texture, g_uv);
    // if (tex_color.a == 0.0) {
    //     discard;
    // }
    // f_color = tex_color * vec4(color, 1.0);

    // vec4 tex_color = texture(circle_texture, g_uv);
    // f_color = tex_color;

    // f_color = texture(circle_texture, g_uv) * vec4(color, 1.0);

    f_color = circle(g_uv) * vec4(color, 1.0);
}
