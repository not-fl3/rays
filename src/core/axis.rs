use na::*;

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Axis {
    X, Y, Z
}
impl Axis {
    pub fn get(&self, v : Vec3<f32>) -> f32 {
        match self {
            &Axis::X => v.x,
            &Axis::Y => v.y,
            &Axis::Z => v.z,
        }
    }
    pub fn set(&self, v : Vec3<f32>, n : f32) -> Vec3<f32> {
        match self {
            &Axis::X => Vec3::new(n  , v.y, v.z),
            &Axis::Y => Vec3::new(v.x,   n, v.z),
            &Axis::Z => Vec3::new(v.x, v.y,   n),
        }
    }
    pub fn get_projected(&self, v : Vec3<f32>) -> Vec3<f32> {
        match self {
            &Axis::X => Vec3::new(v.x, 0.0, v.z),
            &Axis::Y => Vec3::new(v.x, v.y, 0.0),
            &Axis::Z => Vec3::new(0.0, v.y, v.z)
        }
    }
}
