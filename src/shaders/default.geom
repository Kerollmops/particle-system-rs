#version 400

in vec3 v_color[];
layout(points) in;

layout(triangle_strip, max_vertices = 4) out;
out vec3 g_color;
out vec2 g_uv_pos;

void main()
{
    float scale = 0.002;
    g_color = v_color[0];
    vec4 pos = gl_in[0].gl_Position;

    g_uv_pos = vec2(-1.0 * scale, -1.0 * scale);
    gl_Position = pos + vec4(g_uv_pos, 0.0, 0.0);
    EmitVertex();

    g_uv_pos = vec2(1.0 * scale, -1.0 * scale);
    gl_Position = pos + vec4(g_uv_pos, 0.0, 0.0);
    EmitVertex();

    g_uv_pos = vec2(-1.0 * scale, 1.0 * scale);
    gl_Position = pos + vec4(g_uv_pos, 0.0, 0.0);
    EmitVertex();

    g_uv_pos = vec2(1.0 * scale, 1.0 * scale);
    gl_Position = pos + vec4(g_uv_pos, 0.0, 0.0);
    EmitVertex();

    EndPrimitive();
}
