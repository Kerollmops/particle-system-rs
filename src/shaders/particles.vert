#version 400

uniform mat4 matrix;
uniform vec3 eye_pos;
uniform float zfar;

in vec4 position;

out float v_depth;

void main() {
    gl_Position = matrix * vec4(position.xyz, 1.0);
    float dist = distance(eye_pos, gl_Position.xyz);
    v_depth = dist / zfar;
}
