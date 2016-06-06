#version 400

uniform sampler2D circle_texture;
uniform float zfar;

in vec2 g_uv;
in float g_depth;

out vec4 f_color;

vec3 color = vec3(0.17968, 0.79687, 0.44140); // green

// http://wallaceyuen.com/?p=62
void    main() {
    vec4 tex_color = texture(circle_texture, g_uv);
    if (tex_color.a < 1.0) {
        discard;
    }
    else {
        f_color = tex_color * vec4(color, 1.0);
        f_color.w = g_depth;
    }
}
