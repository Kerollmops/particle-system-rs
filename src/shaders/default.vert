#version 400

uniform mat4 matrix;

in vec4 position;

out vec3 v_color;

void main() {
    gl_Position = matrix * vec4(position.xyz, 1.0);
    gl_PointSize = 0.01 * length(gl_Position);
    v_color = vec3(0.17968, 0.79687, 0.44140); // green
    // vColor = vec3(0.796875, 0.1796875, 0.2265625); // red
    // vColor = vec3(1.0, 1.0, 1.0); // white
}
