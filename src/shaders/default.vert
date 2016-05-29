#version 140

uniform mat4 matrix;

in vec4 position;

out vec3 vColor;

void main() {
    gl_Position = vec4(position.xyz, 1.0) * matrix;
    vColor = vec3(1.0, 1.0, 1.0);
}
