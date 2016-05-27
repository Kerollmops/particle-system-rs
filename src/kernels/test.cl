__kernel void multiply_by_scalar(
                __private float const coeff,
                __global float const *const src,
                __global float *const res)
{
    uint const idx = get_global_id(0);
    res[idx] = src[idx] * coeff;
}

__kernel void add_to_each(
    __global float *buff,
    __private float const add)
{
    uint const idx = get_global_id(0);
    buff[idx] += add;
}
