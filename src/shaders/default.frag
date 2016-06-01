#version 400

in vec3 g_color;
in vec2 g_uv_pos;

out vec4 f_color;

void main() {
    vec2 center = vec2(0.0, 0.0);
    float dist = length(center - g_uv_pos);

    if (dist <= 0.001) {
        f_color = vec4(g_color, 1.0);
    }
    else {
        discard;
    }
}
