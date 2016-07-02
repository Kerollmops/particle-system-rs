#version 400

// Bokeh disc.
// by David Hoskins.
// License Creative Commons Attribution-NonCommercial-ShareAlike 3.0 Unported License.

uniform sampler2D tex;
uniform vec2 resolution;
uniform float time;

in vec2 v_tex_coords;

out vec4 f_color;

#define USE_MIPMAP

// The Golden Angle is (3.-sqrt(5.0))*PI radians, which doesn't precompiled for some reason.
// The compiler is a dunce I tells-ya!!
#define GOLDEN_ANGLE 2.39996323

#define ITERATIONS 140

mat2    rot = mat2(cos(GOLDEN_ANGLE), sin(GOLDEN_ANGLE), -sin(GOLDEN_ANGLE), cos(GOLDEN_ANGLE));

vec3    generate_bokeh(sampler2D tex, vec2 uv, float radius, float amount) {
    vec3 acc = vec3(0.0);
    vec3 div = vec3(0.0);
    vec2 pixel = 1.0 / resolution;
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

#define PIXEL_SIZE 4

void    main() {
    vec2 uv = v_tex_coords.xy;
    // float r = 0.8 - 0.8 * cos((time * 0.2 + 0.5) * 6.283);
    // float a = 40.0;
    // uv *= vec2(1.0, -1.0);

    // f_color = vec4(generate_bokeh(tex, uv, r, a), 1.0);
    // f_color = texture(tex, uv);

    vec2 start = floor(uv * resolution / PIXEL_SIZE) * PIXEL_SIZE / resolution;
    f_color = vec4(0);
    for (int x = 0; x < PIXEL_SIZE; x++) {
        for (int y = 0; y < PIXEL_SIZE; y++) {
            vec2 pos = start + (vec2(x, y) / resolution);
            f_color += texture(tex, pos) / (PIXEL_SIZE * PIXEL_SIZE);
        }
    }
}

// uniform sampler2D color_depth_tex;

// const float blurclamp = 3.0; // max blur amount
// const float bias = 0.6; // aperture - bigger values for shallower depth of field
// uniform float focus;    // this value comes from ReadDepth script.

// void main() {
//     vec2 uv = v_tex_coords.xy;
//     vec2 aspectcorrect = resolution.x / resolution.y;
//     float depth = texture2D(color_depth_tex, uv).w;
//     float factor = (depth - focus);
//     vec2 dofblur = vec2 (clamp(factor * bias, -blurclamp, blurclamp));

//     f_color = vec4(0.0);

//     f_color += texture2D(color_depth_tex, uv);
//     f_color += texture2D(color_depth_tex, uv + (vec2(0.0, 0.4) * aspectcorrect) * dofblur);
//     f_color += texture2D(color_depth_tex, uv + (vec2(0.15, 0.37) * aspectcorrect) * dofblur);
//     f_color += texture2D(color_depth_tex, uv + (vec2(0.29, 0.29) * aspectcorrect) * dofblur);
//     f_color += texture2D(color_depth_tex, uv + (vec2(-0.37, 0.15) * aspectcorrect) * dofblur);
//     f_color += texture2D(color_depth_tex, uv + (vec2(0.4, 0.0) * aspectcorrect) * dofblur);
//     f_color += texture2D(color_depth_tex, uv + (vec2(0.37, -0.15) * aspectcorrect) * dofblur);
//     f_color += texture2D(color_depth_tex, uv + (vec2(0.29, -0.29) * aspectcorrect) * dofblur);
//     f_color += texture2D(color_depth_tex, uv + (vec2(-0.15, -0.37) * aspectcorrect) * dofblur);
//     f_color += texture2D(color_depth_tex, uv + (vec2(0.0, -0.4) * aspectcorrect) * dofblur);
//     f_color += texture2D(color_depth_tex, uv + (vec2(-0.15, 0.37) * aspectcorrect) * dofblur);
//     f_color += texture2D(color_depth_tex, uv + (vec2(-0.29, 0.29) * aspectcorrect) * dofblur);
//     f_color += texture2D(color_depth_tex, uv + (vec2(0.37, 0.15) * aspectcorrect) * dofblur);
//     f_color += texture2D(color_depth_tex, uv + (vec2(-0.4, 0.0) * aspectcorrect) * dofblur);
//     f_color += texture2D(color_depth_tex, uv + (vec2(-0.37, -0.15) * aspectcorrect) * dofblur);
//     f_color += texture2D(color_depth_tex, uv + (vec2(-0.29, -0.29) * aspectcorrect) * dofblur);
//     f_color += texture2D(color_depth_tex, uv + (vec2(0.15, -0.37) * aspectcorrect) * dofblur);

//     f_color += texture2D(color_depth_tex, uv + (vec2(0.15, 0.37) * aspectcorrect) * dofblur * 0.9);
//     f_color += texture2D(color_depth_tex, uv + (vec2(-0.37, 0.15) * aspectcorrect) * dofblur * 0.9);
//     f_color += texture2D(color_depth_tex, uv + (vec2(0.37, -0.15) * aspectcorrect) * dofblur * 0.9);
//     f_color += texture2D(color_depth_tex, uv + (vec2(-0.15, -0.37) * aspectcorrect) * dofblur * 0.9);
//     f_color += texture2D(color_depth_tex, uv + (vec2(-0.15, 0.37) * aspectcorrect) * dofblur * 0.9);
//     f_color += texture2D(color_depth_tex, uv + (vec2(0.37, 0.15) * aspectcorrect) * dofblur * 0.9);
//     f_color += texture2D(color_depth_tex, uv + (vec2(-0.37, -0.15) * aspectcorrect) * dofblur * 0.9);
//     f_color += texture2D(color_depth_tex, uv + (vec2(0.15, -0.37) * aspectcorrect) * dofblur * 0.9);

//     f_color += texture2D(color_depth_tex, uv + (vec2(0.29, 0.29) * aspectcorrect) * dofblur * 0.7);
//     f_color += texture2D(color_depth_tex, uv + (vec2(0.4, 0.0) * aspectcorrect) * dofblur * 0.7);
//     f_color += texture2D(color_depth_tex, uv + (vec2(0.29, -0.29) * aspectcorrect) * dofblur * 0.7);
//     f_color += texture2D(color_depth_tex, uv + (vec2(0.0, -0.4) * aspectcorrect) * dofblur * 0.7);
//     f_color += texture2D(color_depth_tex, uv + (vec2(-0.29, 0.29) * aspectcorrect) * dofblur * 0.7);
//     f_color += texture2D(color_depth_tex, uv + (vec2(-0.4, 0.0) * aspectcorrect) * dofblur * 0.7);
//     f_color += texture2D(color_depth_tex, uv + (vec2(-0.29, -0.29) * aspectcorrect) * dofblur * 0.7);
//     f_color += texture2D(color_depth_tex, uv + (vec2(0.0, 0.4) * aspectcorrect) * dofblur * 0.7);

//     f_color += texture2D(color_depth_tex, uv + (vec2(0.29, 0.29) * aspectcorrect) * dofblur * 0.4);
//     f_color += texture2D(color_depth_tex, uv + (vec2(0.4, 0.0) * aspectcorrect) * dofblur * 0.4);
//     f_color += texture2D(color_depth_tex, uv + (vec2(0.29, -0.29) * aspectcorrect) * dofblur * 0.4);
//     f_color += texture2D(color_depth_tex, uv + (vec2(0.0, -0.4) * aspectcorrect) * dofblur * 0.4);
//     f_color += texture2D(color_depth_tex, uv + (vec2(-0.29, 0.29) * aspectcorrect) * dofblur * 0.4);
//     f_color += texture2D(color_depth_tex, uv + (vec2(-0.4, 0.0) * aspectcorrect) * dofblur * 0.4);
//     f_color += texture2D(color_depth_tex, uv + (vec2(-0.29, -0.29) * aspectcorrect) * dofblur * 0.4);
//     f_color += texture2D(color_depth_tex, uv + (vec2(0.0, 0.4) * aspectcorrect) * dofblur * 0.4);

//     f_color = f_color / 41.0;
//     f_color.a = 1.0;
// }
