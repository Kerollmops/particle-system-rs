#version 400

uniform float sphere_diameter;

in vec3 g_color;
in vec2 g_uv_pos;

out vec4 f_color;

void main() {
    vec2 center = vec2(0.0, 0.0);
    float dist = length(center - g_uv_pos);
    float sphere_radius = sphere_diameter / 2.0;

    if (dist <= sphere_radius) {
        f_color = vec4(g_color, 1.0);
    }
    else {
        discard;
    }
}
