#version 140

in vec3 f_Color;

out vec4 Color;

void main() {
    Color = vec4(f_Color, 1.0);
}
