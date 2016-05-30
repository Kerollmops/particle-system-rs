static float back_ease_out(float t, float b, float c, float d)
{
    float s = 1.70158;
    float inner_t = (t / d) - 1.0;
    return (c * (inner_t * inner_t * ((s + 1.0) * inner_t + s) + 1.0) + b);
}

__kernel void update_animation(global float3 const * const from_vec,
                               global float3 const * const to_vec,
                               global float3 * const positions,
                               float time, float duration)
{
    size_t const idx = get_global_id(0);
    float3 const from = from_vec[idx];
    float3 const to = to_vec[idx];

    float progress = back_ease_out(time, from.x, to.x, duration);
    positions[idx] = (float3)(progress, progress, progress);
}

__kernel void init_sphere_animation(global float3 const * const positions,
                                    global float3 * const from_vec,
                                    global float3 * const to_vec,
                                    global float3 * const velocities)
{
    //size_t const size = get_global_size(0);
    size_t const idx = get_global_id(0);

    from_vec[idx] = positions[idx];
    to_vec[idx] = (float3)(-0.5, -0.5, -0.5);
    velocities[idx] = (float3)(0.0, 0.0, 0.0);
}

__kernel void init_cube_animation(global float3 const * const positions,
                                  global float3 * const from_vec,
                                  global float3 * const to_vec,
                                  global float3 * const velocities)
{
    //size_t const size = get_global_size(0);
    size_t const idx = get_global_id(0);

    from_vec[idx] = positions[idx];
    to_vec[idx] = (float3)(0.5, 0.5, 0.5);
    velocities[idx] = (float3)(0.0, 0.0, 0.0);
}

__kernel void update_gravitation(global float3 * const positions,
                                global float3 * const velocities,
                                float3 gravity_point,
                                float t)
{
    size_t const idx = get_global_id(0);
    positions[idx] += gravity_point;
}
