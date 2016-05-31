#version 140

uniform mat4 projection; // projection matrix
uniform mat4 view; // view matrix

in vec4 position;

out vec3 vColor;

void main() {
    gl_Position = projection * view * vec4(position.xyz, 1.0);
    vColor = vec3(0.17968, 0.79687, 0.44140); // green
    // vColor = vec3(0.796875, 0.1796875, 0.2265625); // red
    // vColor = vec3(1.0, 1.0, 1.0); // white
}
