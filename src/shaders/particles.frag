#version 400

uniform sampler2D circle_texture;

in vec2 g_uv;

out vec4 f_color;

vec3 color = vec3(0.17968, 0.79687, 0.44140); // green

void    main() {
    f_color = texture(circle_texture, g_uv) * vec4(color, 1.0);
    if (f_color.a < 1.0) {
        discard;
    }
}
