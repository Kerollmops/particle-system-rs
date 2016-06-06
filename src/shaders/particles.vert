#version 400

uniform mat4 matrix;
uniform float zfar;

in vec4 position;

out float v_depth;

void main() {
    gl_Position = matrix * vec4(position.xyz, 1.0);
    vec3 eye_pos = vec3(1.0, -0.25, -0.5); // FIXME need to get this by uniform
    float dist = distance(eye_pos, position.xyz);
    v_depth = dist / zfar;
}
