#version 400

uniform vec2 resolution;
uniform float circle_diameter;

layout(points) in;
// layout(points) in; v_depth

layout(triangle_strip, max_vertices = 4) out;
out vec2 g_uv;
out float g_depth;

void main() {
    float aspect_ratio = resolution.x / resolution.y;
    vec4 pos = gl_in[0].gl_Position;
    float circle_radius = circle_diameter / 2.0;
    float dist_x = circle_radius;
    float dist_y = circle_radius * aspect_ratio;

    gl_Position = pos + vec4(-dist_x, -dist_y, 0.0, 0.0);
    g_uv = vec2(0.0, 0.0);
    // g_depth = v_depth;
    EmitVertex();

    gl_Position = pos + vec4(dist_x, -dist_y, 0.0, 0.0);
    g_uv = vec2(1.0, 0.0);
    // g_depth = v_depth;
    EmitVertex();

    gl_Position = pos + vec4(-dist_x, dist_y, 0.0, 0.0);
    g_uv = vec2(0.0, 1.0);
    // g_depth = v_depth;
    EmitVertex();

    gl_Position = pos + vec4(dist_x, dist_y, 0.0, 0.0);
    g_uv = vec2(1.0, 1.0);
    // g_depth = v_depth;
    EmitVertex();

    EndPrimitive();
}
