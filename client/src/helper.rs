use std::f32::consts::PI;

pub fn convert_degrees_to_radians(deg: f32) -> f32 {
    return (deg * PI) / 180.0;
}
