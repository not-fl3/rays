use na::*;
use core::axis::*;
use scene::bbox::*;

pub trait Projectable {
    fn get_min(&self, Vec3<f32>) -> Vec3<f32> {
        panic!("you dont need this");
    }
    fn get_max(&self, Vec3<f32>) -> Vec3<f32> {
        panic!("you dont need this");
    }
    fn get_min_bound(&self) -> Vec3<f32> {
        panic!("you dont need this too");
    }
    fn get_max_bound(&self) -> Vec3<f32> {
        panic!("you dont need this too");
    }
    fn square_projected(&self, &Axis) -> f32{
        panic!("you dont need this too");
    }
    fn in_box(&self, BBox) -> bool {
        panic!(":(");
    }
}
