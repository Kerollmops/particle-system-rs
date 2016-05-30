static float back_ease_out(float t, float b, float c, float d) {
    float s = 1.70158;
    float inner_t = (t / d) - 1.0;
    return (c * (inner_t * inner_t * ((s + 1.0) * inner_t + s) + 1.0) + b);
}

static float elastic_ease_out(float t, float b, float c, float d) {
    if (t == 0.0) {
        return b;
    }

    float inner_t = t / d;
    if (inner_t == 1.0) {
        return b + c;
    }

    float p = d * 0.3;
    float a = c;
    float s = p / 4.0;
    float temp = (inner_t * d - s) * (2.0 * M_PI) / p;
    return (a * pow(2.0, -10.0 * inner_t) * sin(temp) + c + b);
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

__kernel void init_sphere_animation(global float3 const * const restrict positions,
                                    global float3 * const restrict from_vec,
                                    global float3 * const restrict to_vec,
                                    global float3 * const restrict velocities) {
    //size_t const size = get_global_size(0);
    size_t const idx = get_global_id(0);

    from_vec[idx] = positions[idx];
    to_vec[idx] = (float3)(-0.25, 0.56, 0.0);
    velocities[idx] = (float3)(0.0, 0.0, 0.0);
}

__kernel void init_cube_animation(global float3 const * const restrict positions,
                                  global float3 * const restrict from_vec,
                                  global float3 * const restrict to_vec,
                                  global float3 * const restrict velocities) {
    size_t const idx = get_global_id(0);

    size_t const number_particles = get_global_size(0);
    size_t const side_particles = cbrt((float)number_particles); // FIXME compute this one time
    size_t const particles_left = number_particles - (side_particles * side_particles * side_particles);
    float const spacing = 200.0 / (float)side_particles;

    from_vec[idx] = positions[idx];
    to_vec[idx] = (float3)((idx / (side_particles * side_particles)) * spacing,
                           ((idx / side_particles) % side_particles) * spacing,
                           (idx % side_particles) * spacing);
    velocities[idx] = (float3)(0.0, 0.0, 0.0);
}

__kernel void update_gravitation(global float3 * const restrict positions,
                                 global float3 * const restrict velocities,
                                 float3 gravity_point,
                                 float t) {
    size_t const idx = get_global_id(0);
    positions[idx] += gravity_point;
}
