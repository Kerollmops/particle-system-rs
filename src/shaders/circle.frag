#version 400

uniform vec2 resolution;
uniform float circle_diameter;

in vec2 v_uv;

out vec4 f_color;

void    main() {
    float aspect_ratio = resolution.x / resolution.y;
    vec2 pos = vec2(v_uv.x * aspect_ratio, v_uv.y);
    float dist = length(pos);
    float circle_radius = circle_diameter / 2.0;

    if (dist > circle_radius) {
        discard;
    }
    f_color = vec4(1.0, 1.0, 1.0, 1.0);
}
