#version 400

uniform vec2 resolution;
uniform float circle_diameter;

in vec2 v_uv;

out vec4 f_color;

void    main() {
    vec2 uv = v_uv;
    uv *= 2.0; uv -= 1.0; // in range [-1.0, 1.0]
    vec2 pos = vec2(uv.x, uv.y);
    float dist = length(pos);
    float circle_radius = 1.0;

    if (dist > circle_radius) {
        discard;
    }
    f_color = vec4(1.0, 1.0, 1.0, 1.0);
}
