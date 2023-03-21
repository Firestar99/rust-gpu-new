// build-pass
// compile-flags: -C target-feature=+RuntimeDescriptorArray,+ext:SPV_EXT_descriptor_indexing,ShaderNonUniformEXT,RuntimeDescriptorArrayEXT,SampledImageArrayNonUniformIndexingEXT,StorageBufferArrayNonUniformIndexingEXT

use spirv_std::spirv;
use spirv_std::{Image, RuntimeArray, Sampler, TypedBuffer};

#[spirv(fragment)]
pub fn main(
    #[spirv(frag_coord)] frag_coord: glam::Vec4,
    #[spirv(descriptor_set = 0, binding = 0)] sampler: &Sampler,
    #[spirv(descriptor_set = 0, binding = 1)] slice: &RuntimeArray<Image!(2D, type=f32, sampled)>,
    #[spirv(descriptor_set = 1, binding = 0, storage_buffer)] buffer_slice: &RuntimeArray<
        TypedBuffer<glam::Vec4>,
    >,
    output: &mut glam::Vec4,
) {
    let idx_image = frag_coord.as_uvec4().x as usize;
    let img = unsafe { slice.index_nonuniform(idx_image) };
    let v2 = glam::Vec2::new(0.0, 1.0);
    let r1: glam::Vec4 = img.sample(*sampler, v2);

    let idx_buffer = frag_coord.as_uvec4().y as usize;
    let r2: glam::Vec4 = **unsafe { buffer_slice.index_nonuniform(idx_buffer) };

    *output = r1 + r2;
}
