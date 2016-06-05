#version 400

// Bokeh disc.
// by David Hoskins.
// License Creative Commons Attribution-NonCommercial-ShareAlike 3.0 Unported License.

uniform sampler2D color_texture;
uniform sampler2D depth_texture;
uniform vec2 resolution;
uniform float time;

in vec2 v_tex_coords;

out vec4 f_color;

// #define USE_MIPMAP

// The Golden Angle is (3.-sqrt(5.0))*PI radians, which doesn't precompiled for some reason.
// The compiler is a dunce I tells-ya!!
#define GOLDEN_ANGLE 2.39996323

#define ITERATIONS 140
// #define ITERATIONS 50


mat2 rot = mat2(cos(GOLDEN_ANGLE), sin(GOLDEN_ANGLE), -sin(GOLDEN_ANGLE), cos(GOLDEN_ANGLE));

float   interpolate_focus(float pos, float val[3]) {
    if (pos <= 0.5) { // [0.0; 0.5]
        pos = pos / 0.5;
        return val[1] * pos + val[0] * (1.0 - pos);
    }
    else { // (0.5; 1.0]
        pos = (pos - 0.5) / 0.5;
        return val[2] * pos + val[1] * (1.0 - pos);
    }
}

vec4    generate_bokeh(sampler2D color_tex, sampler2D depth_tex, vec2 uv) {
    vec4 acc = vec4(0.0);
    vec4 div = vec4(0.0);
    vec2 pixel = 1.0 / resolution;
    float r = 1.0;
    float radius = 0.0;
    float amount = 40.0;
    vec2 vangle = vec2(0.0, radius); // Start angle
    amount += radius * 500.0;

    for (int j = 0; j < ITERATIONS; j++) {
        r += 1.0 / r;
        vangle = rot * vangle;
        // (r - 1.0) here is the equivalent to sqrt(0, 1, 2, 3...)
        #ifdef USE_MIPMAP
            vec4 col = texture(color_tex, uv + pixel * (r - 1.0) * vangle, radius * 1.25);
            float depth = texture(depth_tex, uv + pixel * (r - 1.0) * vangle, radius * 1.25).x;
        #else
            vec4 col = texture(color_tex, uv + pixel * (r - 1.0) * vangle);
            float depth = texture(depth_tex, uv + pixel * (r - 1.0) * vangle).x;
        #endif
        // col = col * col * 1.5; // ...contrast it for better highlights - leave this out elsewhere.
        vec4 bokeh = pow(col, vec4(9.0)) * amount + 0.4;
        acc += col * vec4(bokeh.rgb * col.a, 1.0) * col.a;
        div += vec4(bokeh.rgb * col.a, 1.0);
    }
    return acc / div;
}

void    main() {
    vec2 uv = v_tex_coords.xy;
    uv *= vec2(1.0, -1.0);
    // float r = 0.8 - 0.8 * cos((time * 0.2 + 0.5) * 6.283);
    // float r = cos(time);
    // float r = (1.0 - texture(depth_texture, uv).x) * 10.0;
    // float r = 0.5;
    float a = 40.0;

    // float focus_values[3] = float[](1.0, 0.0, 1.0);
    // float r = interpolate_focus(0.5, focus_values);

    // float r = interpolate_focus(texture(depth_texture, uv).x, focus_values);

    f_color = generate_bokeh(color_texture, depth_texture, uv);
    // f_color = vec4(vec3((1.0 - texture(depth_texture, uv).x) * 100.0, 0.0, 0.0), 1.0);
}
