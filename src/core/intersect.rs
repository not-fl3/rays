use na::*;
use core::ray::*;
use materials::*;

#[derive(Clone)]
pub struct IntersectData {
    pub normal   : Vec3<f32>,
    pub pos      : Vec3<f32>,
    pub material : BoxedMaterial,
    pub uv       : Vec2<f32>
}

pub trait Intersectable {
    fn intersect(&self, Ray) -> Option<(f32, IntersectData)>;
}

#[allow(dead_code)]
pub fn gen_test_intersectable() -> IntersectData {
    IntersectData {
        normal   : Vec3::new(0.0, 1.0, 0.0),
        pos      : Vec3::new(0.0, 0.0, 0.0),
        uv       : Vec2::new(0.0, 0.0),
        material : test_mat()
    }
}
