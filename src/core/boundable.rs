use core::*;

use std::f32::MAX;

pub trait Boundable {
    fn get_bound_box(&self) -> BBox;
}

impl Boundable for Vec<BoxedObject> {
    fn get_bound_box(&self) -> BBox {
        let mut bmin = Vec3::new(MAX, MAX, MAX);
        let mut bmax = Vec3::new(-MAX, -MAX, -MAX);

        for obj in self {
            bmin = obj.get_min(bmin);
            bmax = obj.get_max(bmax);
        }
        BBox {bmin : bmin, bmax : bmax}
    }
}
