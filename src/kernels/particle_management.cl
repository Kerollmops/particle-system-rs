__kernel void add_to_each(
    __global float *buff,
    __private float const add)
{
    uint const idx = get_global_id(0);
    buff[idx] += add;
}

__kernel void init_sphere(__global float *positions, __global float *velocities)
{
    //
}

__kernel void init_cube(__global float *positions, __global float *velocities)
{
    //
}

__kernel void update(__global float *positions, __global float *velocities,
                float3 gravity_point)
{
    //
}
