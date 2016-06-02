#version 400

uniform float circle_diameter;
uniform float aspect_ratio;

in vec3 g_color;
in vec2 g_uv_pos;

out vec4 f_color;

// https://github.com/mattdesl/lwjgl-basics/wiki/ShaderLesson3
void main() {
    vec2 center = vec2(0.0, 0.0);
    vec2 pos = vec2(g_uv_pos.x * aspect_ratio, g_uv_pos.y);
    float dist = length(center - pos);
    float circle_radius = circle_diameter / 2.0;

    // https://en.wikibooks.org/wiki/OpenGL_Programming/Depth_of_Field
    // + motion blur
    // https://www.shadertoy.com/view/XdXXz4
    // https://www.shadertoy.com/view/MdSGDm
    // + shadows...

    if (dist <= circle_radius) {
        f_color = vec4(g_color, 1.0);
    }
    else {
        discard;
    }
}
