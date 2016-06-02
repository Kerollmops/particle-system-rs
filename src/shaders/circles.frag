#version 400

uniform float circle_diameter;
uniform float aspect_ratio;
uniform float time;

in vec3 g_color;
in vec2 g_uv_pos;

out vec4 f_color;

vec3    circle_texture(vec2 uv) {
    vec2 center = vec2(0.0, 0.0);
    vec2 pos = vec2(uv.x * aspect_ratio, uv.y);
    float dist = length(center - pos);
    float circle_radius = circle_diameter / 2.0;

    if (dist > circle_radius) {
        discard;
    }
    return g_color;
}

void    main() {
    // https://github.com/mattdesl/lwjgl-basics/wiki/ShaderLesson3
    // https://en.wikibooks.org/wiki/OpenGL_Programming/Depth_of_Field
    // + motion blur
    // https://www.shadertoy.com/view/XdXXz4
    // https://www.shadertoy.com/view/MdSGDm
    // + shadows...

    f_color = vec4(circle_texture(g_uv_pos), 1.0);
}
