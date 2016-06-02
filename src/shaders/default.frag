#version 400

uniform float circle_diameter;
uniform float aspect_ratio;
uniform float time;

in vec3 g_color;
in vec2 g_uv_pos;

out vec4 f_color;

vec3    circle_texture(vec2 uv) {
    vec2 center = vec2(0.0, 0.0);
    vec2 pos = vec2(uv.x * aspect_ratio, uv.y);
    float dist = length(center - pos);
    float circle_radius = circle_diameter / 2.0;

    if (dist > circle_radius) {
        // discard;
        return vec3(0.0); // background color
    }
    return g_color;
}

// vec3    texture(vec2 uv) {
//     return texture2D(iChannel0, vec2(uv.x,1.-uv.y)).rgb;
//     //return texture2D(iChannel0,uv).rgb;
// }

float   grid(float var, float size) {
    return floor(var * size) / size;
}

float   rand(vec2 co){
    return fract(sin(dot(co.xy, vec2(12.9898,78.233))) * 43758.5453);
}

vec4    blur(vec2 uv) {

    // float time = iGlobalTime;
    // vec2 uv = (fragCoord.xy / iResolution.xy);

    float bluramount = sin(time) * 0.1;
    // if (iMouse.w >= 1.) {
    //     bluramount = (iMouse.x / iResolution.x) / 10.;
    // }

    //float dists = 5.;
    vec3 blurred_image = vec3(0.);
    #define repeats 60.
    for (float i = 0.; i < repeats; i++) {
        vec2 q = vec2(cos(degrees((i / repeats) * 360.)), sin(degrees((i / repeats) * 360.))) * (rand(vec2(i, uv.x + uv.y)) + bluramount);
        vec2 uv2 = uv + (q * bluramount);
        blurred_image += circle_texture(uv2) / 2.;
        //One more to hide the noise.
        q = vec2(cos(degrees((i / repeats) * 360.)), sin(degrees((i / repeats) * 360.))) * (rand(vec2(i + 2., uv.x + uv.y + 24.)) + bluramount);
        uv2 = uv + (q * bluramount);
        blurred_image += circle_texture(uv2) / 2.;
    }
    blurred_image /= repeats;
    return vec4(blurred_image, 1.0);
}

void    main() {
    // https://github.com/mattdesl/lwjgl-basics/wiki/ShaderLesson3
    // https://en.wikibooks.org/wiki/OpenGL_Programming/Depth_of_Field
    // + motion blur
    // https://www.shadertoy.com/view/XdXXz4
    // https://www.shadertoy.com/view/MdSGDm
    // + shadows...
    // f_color = BlurH(/* vec2(1024.0 * 40.0, 768.0),  */g_uv_pos, 20.0);
    f_color = blur(g_uv_pos);
}
