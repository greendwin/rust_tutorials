pub fn clamp(val: f32, src_min: f32, src_max: f32, dst_min: f32, dst_max: f32) -> f32 {
    val
}

pub fn clamp_f2u8(val: f32) -> u8 {
    clamp(val, 0.0, 1.0, 0.0, 255.9999) as u8
}

// TODO: test me
