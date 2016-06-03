#version 400

uniform mat4 matrix;

in vec4 position;

void main() {
    gl_Position = matrix * vec4(position.xyz, 1.0);
}
