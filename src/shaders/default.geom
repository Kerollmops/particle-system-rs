#version 400

uniform mat4 matrix;
uniform float aspect_ratio;
uniform float circle_diameter;

in vec3 v_color[];
layout(points) in;

layout(triangle_strip, max_vertices = 4) out;
out vec3 g_color;
out vec2 g_uv_pos;

void main()
{
    g_color = v_color[0];
    vec4 pos = gl_in[0].gl_Position;
    float circle_radius = (circle_diameter)  / 2.0;
    float dist_x = circle_radius;
    float dist_y = circle_radius * aspect_ratio;

    g_uv_pos = vec2(-dist_x, -dist_y);
    gl_Position = pos + vec4(g_uv_pos, 0.0, 0.0);
    EmitVertex();

    g_uv_pos = vec2(dist_x, -dist_y);
    gl_Position = pos + vec4(g_uv_pos, 0.0, 0.0);
    EmitVertex();

    g_uv_pos = vec2(-dist_x, dist_y);
    gl_Position = pos + vec4(g_uv_pos, 0.0, 0.0);
    EmitVertex();

    g_uv_pos = vec2(dist_x, dist_y);
    gl_Position = pos + vec4(g_uv_pos, 0.0, 0.0);
    EmitVertex();

    EndPrimitive();
}
