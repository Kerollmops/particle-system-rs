__kernel void init_sphere(__global float3 *positions, __global float3 *velocities)
{
    //
}

__kernel void init_cube(global float4 * const positions, global float4 * const velocities)
{
    size_t const size = get_global_size(0);
    size_t const idx = get_global_id(0);
    positions[idx] = (float4)(0.0f, 0.0f, 0.0f, 0.0f);
    //positions[idx] = 10.f;

}

__kernel void update(global float4 * const positions, global float4 * const velocities,
                     float4 gravity_point) // TODO add time counter
{
    size_t const idx = get_global_id(0);
    positions[idx] += gravity_point;
}
