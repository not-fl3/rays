use na::*;

pub enum Light {
    PointLight(Vec3<f32>, Vec3<f32>)
}


#[allow(dead_code)]
pub fn point_light_white(v : Vec3<f32>) -> Light {
    Light::PointLight(v, Vec3::new(1.0, 1.0, 1.0))
}

