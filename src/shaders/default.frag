#version 400

uniform float circle_diameter;
uniform float aspect_ratio;
uniform float time;

in vec3 v_color;

out vec4 f_color;

void    main() {
    // https://github.com/mattdesl/lwjgl-basics/wiki/ShaderLesson3
    // https://en.wikibooks.org/wiki/OpenGL_Programming/Depth_of_Field
    // + motion blur
    // https://www.shadertoy.com/view/XdXXz4
    // https://www.shadertoy.com/view/MdSGDm
    // + shadows...

    f_color = vec4(v_color, 1.0);
}
