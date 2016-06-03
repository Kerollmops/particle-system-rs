#version 400

uniform mat4 matrix;

in vec2 position;
in vec2 tex_coords;

out vec2 v_uv;

void main() {
    gl_Position = matrix * vec4(position, 0.0, 1.0);
    v_uv = tex_coords;
}
