#version 400

// Bokeh disc.
// by David Hoskins.
// License Creative Commons Attribution-NonCommercial-ShareAlike 3.0 Unported License.

uniform sampler2D tex;
uniform float time;

in vec2 v_tex_coords;

out vec4 f_color;


// #define USE_MIPMAP

// The Golden Angle is (3.-sqrt(5.0))*PI radians, which doesn't precompiled for some reason.
// The compiler is a dunce I tells-ya!!
#define GOLDEN_ANGLE 2.39996323

#define ITERATIONS 140

vec2 iResolution = vec2(1024.0, 768.0); // FIXME

mat2 rot = mat2(cos(GOLDEN_ANGLE), sin(GOLDEN_ANGLE), -sin(GOLDEN_ANGLE), cos(GOLDEN_ANGLE));

//-------------------------------------------------------------------------------------------
vec3    Bokeh(sampler2D tex, vec2 uv, float radius, float amount) {

    vec3 acc = vec3(0.0);
    vec3 div = vec3(0.0);
    vec2 pixel = 1.0 / iResolution.xy;
    float r = 1.0;
    vec2 vangle = vec2(0.0, radius); // Start angle
    amount += radius * 500.0;

    for (int j = 0; j < ITERATIONS; j++)
    {
        r += 1. / r;
        vangle = rot * vangle;
        // (r - 1.0) here is the equivalent to sqrt(0, 1, 2, 3...)
        #ifdef USE_MIPMAP
            vec3 col = texture(tex, uv + pixel * (r - 1.0) * vangle, radius * 1.25).xyz;
        #else
            vec3 col = texture(tex, uv + pixel * (r - 1.0) * vangle).xyz;
        #endif
        col = col * col * 1.5; // ...contrast it for better highlights - leave this out elsewhere.
        vec3 bokeh = pow(col, vec3(9.0)) * amount+.4;
        acc += col * bokeh;
        div += bokeh;
    }
    return acc / div;
}

void    main() {
    // vec2 uv = fragCoord.xy / iResolution.xy;
    vec2 uv = v_tex_coords.xy;
    // vec2 uv = vec2(v_tex_coords.x, v_tex_coords.y);
    float r = 0.8 - 0.8 * cos((time * 0.2 + 0.5) * 6.283);

    float a = 40.0;
    uv *= vec2(1.0, -1.0);

    f_color = vec4(Bokeh(tex, uv, r, a), 1.0);

    // f_color = texture(tex, v_tex_coords);
}
