#version 400

uniform vec2 resolution;
uniform float circle_diameter;

layout(points) in;

layout(triangle_strip, max_vertices = 4) out;
out vec2 g_uv;

void main() {
    float aspect_ratio = resolution.x / resolution.y;
    vec4 pos = gl_in[0].gl_Position;
    float circle_radius = circle_diameter / 2.0;
    float dist_x = circle_radius;
    float dist_y = circle_radius * aspect_ratio;

    g_uv = vec2(-dist_x, -dist_y);
    gl_Position = pos + vec4(g_uv, 0.0, 0.0);
    EmitVertex();

    g_uv = vec2(dist_x, -dist_y);
    gl_Position = pos + vec4(g_uv, 0.0, 0.0);
    EmitVertex();

    g_uv = vec2(-dist_x, dist_y);
    gl_Position = pos + vec4(g_uv, 0.0, 0.0);
    EmitVertex();

    g_uv = vec2(dist_x, dist_y);
    gl_Position = pos + vec4(g_uv, 0.0, 0.0);
    EmitVertex();

    EndPrimitive();
}
