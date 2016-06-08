#version 400

uniform mat4 projection;
uniform mat4 view;
uniform vec3 eye_pos;
uniform float znear; // delete
uniform float zfar;

in vec4 position;

out float v_depth;

void main() {
    vec4 view_pos = view * vec4(position.xyz, 1.0);
    float dist = length(view_pos.xyz);

    gl_Position = projection * view_pos;
    v_depth = dist / zfar;
}
