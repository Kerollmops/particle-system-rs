#version 140

uniform Locals {
    mat4 u_World;
    mat4 u_Model;
    mat4 u_View;
};

in vec3 v_Position;

out vec3 f_Color;

void main() {
    gl_Position = matrix * vec4(position.xyz, 1.0);
    vColor = vec3(0.17968, 0.79687, 0.44140); // green
    // vColor = vec3(0.796875, 0.1796875, 0.2265625); // red
    // vColor = vec3(1.0, 1.0, 1.0); // white
}
