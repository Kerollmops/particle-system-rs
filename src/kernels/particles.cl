float quad_ease_in_out(float t, float b, float c, float d) {
    float inner_t = t / (d / 2.0);
    if (inner_t < 1.0) {
        return (c / 2.0 * (pow(inner_t, 2))) + b;
    }
    float temp = inner_t - 1.0;
    return (-c / 2.0 * (((inner_t - 2.0) * (temp)) - 1.0) + b);
}

static float back_ease_out(float t, float b, float c, float d) {
    float s = 1.70158f;
    float inner_t = (t / d) - 1.0f;
    return (c * (inner_t * inner_t * ((s + 1.0f) * inner_t + s) + 1.0f) + b);
}

static float elastic_ease_out(float t, float b, float c, float d) {
    if (t == 0.0f) {
        return b;
    }

    float inner_t = t / d;
    if (inner_t == 1.0f) {
        return b + c;
    }

    float p = d * 0.3f;
    float a = c;
    float s = p / 4.0f;
    float temp = (inner_t * d - s) * (2.0f * M_PI_F) / p;
    return (a * pow(2.0f, -10.0f * inner_t) * sin(temp) + c + b);
}

#define EASING_ANIMATION elastic_ease_out

__kernel void update_animation(global float3 const * const restrict from_vec,
                               global float3 const * const restrict to_vec,
                               global float3 * const restrict positions,
                               float time, float duration) {
    size_t const idx = get_global_id(0);
    float3 const from = from_vec[idx];
    float3 const to = to_vec[idx];

    positions[idx] = (float3)(EASING_ANIMATION(time, from.x, to.x - from.x, duration),
                              EASING_ANIMATION(time, from.y, to.y - from.y, duration),
                              EASING_ANIMATION(time, from.z, to.z - from.z, duration));
}

static size_t  xorshift64star(size_t x) {
    x ^= x >> 12; // a
    x ^= x << 25; // b
    x ^= x >> 27; // c
    return x * size_t(2685821657736338717);
}

__kernel void init_rand_sphere_animation(global float3 const * const restrict positions,
                                         global float3 * const restrict from_vec,
                                         global float3 * const restrict to_vec,
                                         global float3 * const restrict velocities) {
    size_t const idx = get_global_id(0);

    float const scaling = 1.f / 20.f;
    size_t const radius = 10;

    size_t const scal_rad = (radius / 2) * 10000;
    float const u = radians((float)(xorshift64star(idx >> 3) % 360));
    float const v = radians((float)(xorshift64star(idx << 2) % 360));
    float const radius_rand = ((float)(xorshift64star(idx >> 2) % scal_rad)
                                + ((radius * 10000) - scal_rad)) / 10000.f;

    // http://www.wolframalpha.com/input/?i=sphere
    float const x = radius_rand * cos(u) * sin(v);
    float const y = radius_rand * sin(u) * sin(v);
    float const z = radius_rand * cos(v);

    from_vec[idx] = positions[idx];
    to_vec[idx] = (float3)(x, y, z);
    to_vec[idx] *= scaling;
    velocities[idx] = (float3)(0.0f, 0.0f, 0.0f);
}

__kernel void init_rand_cube_animation(global float3 const * const restrict positions,
                                       global float3 * const restrict from_vec,
                                       global float3 * const restrict to_vec,
                                       global float3 * const restrict velocities) {
    size_t const idx = get_global_id(0);

    float const scaling = 1.f / 20.f;
    size_t const radius = 10;

    size_t const diameter = radius * 2;
    float3 const center = (float3)(10.f, 10.f, 10.f);
    float const x = (float)(xorshift64star(idx << 3) % (diameter * 10000)) / 10000.f;
    float const y = (float)(xorshift64star(idx >> 2) % (diameter * 10000)) / 10000.f;
    float const z = (float)(xorshift64star(idx << 2) % (diameter * 10000)) / 10000.f;

    from_vec[idx] = positions[idx];
    to_vec[idx] = (float3)(x, y, z);
    to_vec[idx] -= center;
    to_vec[idx] *= scaling;
    velocities[idx] = (float3)(0.0f, 0.0f, 0.0f);
}

__kernel void init_cube_animation(global float3 const * const restrict positions,
                                  global float3 * const restrict from_vec,
                                  global float3 * const restrict to_vec,
                                  global float3 * const restrict velocities) {
    size_t const idx = get_global_id(0);
    size_t const total_parts = get_global_size(0);

    size_t const side_parts = cbrt((float)total_parts); // FIXME compute this one time
    size_t const parts_left = total_parts - (side_parts * side_parts * side_parts);
    float const spacing = (1.f / (float)side_parts);

    from_vec[idx] = positions[idx];
    if (idx >= total_parts - parts_left) {
        size_t const rand_idx = xorshift64star(idx << 3) % (total_parts - parts_left);
        to_vec[idx] = (float3)((rand_idx / (side_parts * side_parts)) * spacing,
                               ((rand_idx / side_parts) % side_parts) * spacing,
                               (rand_idx % side_parts) * spacing);
    }
    else {
        to_vec[idx] = (float3)((idx / (side_parts * side_parts)) * spacing,
                               ((idx / side_parts) % side_parts) * spacing,
                               (idx % side_parts) * spacing);
    }
    to_vec[idx] -= ((float)(side_parts - 1) / 2.0f) * spacing;
    velocities[idx] = (float3)(0.0f, 0.0f, 0.0f);
}

__kernel void update_gravitation(global float3 * const restrict positions,
                                 global float3 * const restrict velocities,
                                 float3 gravity_point,
                                 float t) {
    size_t const idx = get_global_id(0);
    positions[idx] += gravity_point;
}
