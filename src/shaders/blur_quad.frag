#version 400

// Bokeh disc.
// by David Hoskins.
// License Creative Commons Attribution-NonCommercial-ShareAlike 3.0 Unported License.

uniform sampler2D col_depth_tex;
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

// float   interpolate_focus(float pos, float middle, float val[3]) {
//     if (pos <= middle) { // [0.0; middle]
//         pos = pos / middle;
//         return val[1] * pos + val[0] * (1.0 - pos);
//     }
//     else { // (middle; 1.0]
//         pos = (pos - middle) / middle;
//         return val[2] * pos + val[1] * (1.0 - pos);
//     }
// }

// vec4    generate_bokeh(sampler2D tex, vec2 uv) {
//     float focus_values[3] = float[](1.0, 0.0, 1.0);
//     vec4 acc = vec4(0.0);
//     vec4 div = vec4(0.0);
//     vec2 pixel = 1.0 / resolution;
//     float r = 1.0;
//     float amount = 40.0;

//     float depth = texture(tex, uv).w;
//     float focus = interpolate_focus(depth, 0.5, focus_values);
//     float radius = focus;
//     vec2 vangle = vec2(0.0, radius); // Start angle
//     amount += radius * 500.0;

//     for (int j = 0; j < ITERATIONS; j++) {
//         r += 1.0 / r;
//         vangle = rot * vangle;
//         // vec4 col = vec4(vec3((1.0 - depth_col) * 100.0), 1.0);
//         vec4 col = texture(tex, uv + pixel * (r - 1.0) * vangle);
//         // vec4 col = vec4(vec3(focus), 1.0);

//         vec4 bokeh = pow(col, vec4(9.0)) * amount + 0.4;
//         acc += col * vec4(bokeh.rgb * col.a, 1.0) * col.a;
//         div += vec4(bokeh.rgb * col.a, 1.0);
//     }
//     return acc / div;
// }

// vec4    generate_bokeh(sampler2D tex, vec2 uv) {
//     float focus_values[3] = float[](1.0, 0.0, 1.0);
//     float depth = texture(tex, uv).w;
//     float focus = interpolate_focus(depth, 0.5, focus_values);
//     float radius = focus;
//     float amount = 40.0;
//     vec3 acc = vec3(0.0);
//     vec3 div = vec3(0.0);
//     vec2 pixel = 1.0 / iResolution.xy;
//     float r = 1.0;
//     vec2 vangle = vec2(0.0,radius); // Start angle
//     amount += radius*500.0;

//     for (int j = 0; j < ITERATIONS; j++)
//     {
//         r += 1. / r;
//         vangle = rot * vangle;
//         // (r-1.0) here is the equivalent to sqrt(0, 1, 2, 3...)
//         #ifdef USE_MIPMAP
//             vec3 col = texture2D(tex, uv + pixel * (r-1.) * vangle, radius*1.25).xyz;
//         #else
//             vec3 col = texture2D(tex, uv + pixel * (r-1.) * vangle).xyz;
//         #endif
//         col = col * col * 1.5; // ...contrast it for better highlights - leave this out elsewhere.
//         vec3 bokeh = pow(col, vec3(9.0)) * amount+.4;
//         acc += col * bokeh;
//         div += bokeh;
//     }
//     return acc / div;
// }

float   interpolate_focus(float pos, float middle, float val[3]) {
    if (pos <= middle) { // [0.0; middle]
        pos = pos / middle;
        return mix(val[0], val[1], pos);
    }
    else { // ]middle; 1.0]
        pos = (pos - middle) / (1.0 - middle);
        return mix(val[1], val[2], pos);
    }
}

// vec4    generate_bokeh(sampler2D tex, vec2 uv) {
//     float focus_values[3] = float[](1.0, 0.0, 1.0);
//     vec4 acc = vec4(0.0);
//     vec4 div = vec4(0.0);
//     vec2 pixel = 1.0 / resolution;
//     float r = 1.0;
//     float a = 40.0;

//     float depth = texture(tex, uv).w;
//     if (depth == 1.0) {
//         return vec4(vec3(0.0), 1.0);
//     }
//     // else if (depth > 0.2) {
//     //     return vec4(vec3(interpolate_focus(depth, 0.5, focus_values)), 1.0);
//     // }
//     // float middle = (sin(time) + 1.0) / 2.0;
//     float focus = interpolate_focus(depth, 0.5, focus_values);
//     float radius = focus;
//     vec2 vangle = vec2(0.0, radius); // Start angle
//     float amount = a + radius * 500.0;

//     for (int j = 0; j < ITERATIONS; j++) {
//         r += 1.0 / r;

//         // depth = texture(tex, uv + pixel * (r - 1.0)).w;
//         // focus = interpolate_focus(depth, 0.5, focus_values);
//         // radius = focus;
//         // // vangle = vec2(0.0, radius);
//         // amount = a + radius * 500.0;

//         vangle = rot * vangle;
//         vec4 col = vec4(texture(tex, uv + pixel * (r - 1.0) * vangle).rgb, 1.0);
//         // vec4 col = vec4(vec3((1.0 - texture(tex, uv + pixel * (r - 1.0) * vangle).w)), 1.0);
//         vec4 bokeh = pow(col, vec4(9.0)) * amount + 0.4;
//         acc += col * vec4(bokeh.rgb * col.a, 1.0) * col.a;
//         div += vec4(bokeh.rgb * col.a, 1.0);
//     }
//     return acc / div;
// }

// void    main() {
//     vec2 uv = v_tex_coords.xy;
//     uv *= vec2(1.0, -1.0);

//     f_color = generate_bokeh(col_depth_tex, uv);
// }

vec4    generate_bokeh(sampler2D tex, vec2 uv, float radius, float zed) {
    vec4 acc = vec4(0.0), div = acc;
    vec2 pixel = 1.0 / resolution;
    float r = 1.0;
    vec2 vangle = vec2(0.0, radius); // Start angle
    for (int j = 0; j < ITERATIONS; j++) {
        r += 1. / r;
        vangle *= rot;
        vec4 col = vec4(texture(tex, uv + pixel * (r - 1.0) * vangle).rgb, 1.0);
        // float dim = smoothstep(1.0, 0.0, zed - col.w); // hermite interpolation
        vec4 bokeh = (pow(col, vec4(9.0)) * 200.0 + 0.4) /* * col.w */; // ..Varies depending on intensity needed..

        acc += col * bokeh;
        div += bokeh;
    }
    return acc / div;
}

// https://www.shadertoy.com/view/ldG3W3
void    main() {
    float focus_values[3] = float[](1.0, 0.0, 1.0);
    vec2 uv = v_tex_coords.xy;
    uv *= vec2(1.0, -1.0);

    float depth = texture(col_depth_tex, uv).w;
    float focus = interpolate_focus(depth, 0.5, focus_values);
    // depth = min(depth, 150.0);

    f_color = generate_bokeh(col_depth_tex, uv, focus, focus);
}
