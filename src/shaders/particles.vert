#version 400

uniform mat4 matrix;

in vec4 position;

// out float v_depth;

void main() {
    gl_Position = matrix * vec4(position.xyz, 1.0);
    // v_depth = position.z / zfar;
}
