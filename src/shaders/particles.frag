#version 400

uniform sampler2D circle_texture;
uniform float time;

in vec2 g_uv_pos;

out vec4 f_color;

vec3 color = vec3(0.17968, 0.79687, 0.44140); // green
// vec3 color = vec3(0.796875, 0.1796875, 0.2265625); // red
// vec3 color = vec3(1.0, 1.0, 1.0); // white

// vec4    circle_texture(vec2 v_uv) {
//     float aspect_ratio = 1024.0 / 768.0;
//     vec2 pos = vec2(v_uv.x * aspect_ratio, v_uv.y);
//     float dist = length(pos);
//     float circle_radius = 0.002 / 2.0;

//     if (dist > circle_radius) {
//         discard;
//     }
//     return vec4(1.0, 1.0, 1.0, 1.0);
// }

void    main() {
    // https://github.com/mattdesl/lwjgl-basics/wiki/ShaderLesson3
    // https://en.wikibooks.org/wiki/OpenGL_Programming/Depth_of_Field
    // + motion blur
    // https://www.shadertoy.com/view/XdXXz4
    // https://www.shadertoy.com/view/MdSGDm
    // + shadows...

    // FIXME need to use mipmaping here !
    f_color = texture(circle_texture, g_uv_pos) * vec4(color, 1.0);
    // f_color = circle_texture(g_uv_pos) * vec4(color, 1.0);
}
