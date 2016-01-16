use core::*;

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct BBox {
    pub bmin : Vec3<f32>,
    pub bmax : Vec3<f32>
}

impl BBox {
    pub fn intersect(&self, r : Ray) -> Option<f32> {
        let t1 = (self.bmin.x - r.pos.x) / r.dir.x;
        let t2 = (self.bmax.x - r.pos.x) / r.dir.x;
        let t3 = (self.bmin.y - r.pos.y) / r.dir.y;
        let t4 = (self.bmax.y - r.pos.y) / r.dir.y;
        let t5 = (self.bmin.z - r.pos.z) / r.dir.z;
        let t6 = (self.bmax.z - r.pos.z) / r.dir.z;

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        if tmax < 0.0 {
            return None;
        }

        if tmin > tmax {
             return None;
        }
        return Some(tmin);
    }

    pub fn split_left(&self, x : f32, axis : Axis) -> BBox {
        BBox { bmin : self.bmin, bmax : axis.set(self.bmax, x) }
    }
    pub fn split_right(&self, x : f32, axis : Axis) -> BBox {
        BBox { bmin : axis.set(self.bmin, x), bmax : self.bmax }
    }
    pub fn overlap(&self, bbox : &BBox) -> bool {
        if self.bmin.x > bbox.bmax.x {
            return false;
        }
        if self.bmax.x < bbox.bmin.x {
            return false;
        }
        if self.bmin.y > bbox.bmax.y {
            return false;
        }
        if self.bmax.y < bbox.bmin.y {
            return false;
        }
        if self.bmin.z > bbox.bmax.z {
            return false;
        }
        if self.bmax.z < bbox.bmin.z {
            return false;
        }
        return true;

    }
    pub fn in_box(&self, v : Vec3<f32>) -> bool {
        v.x >= self.bmin.x && v.x <= self.bmax.x &&
            v.y >= self.bmin.y && v.y <= self.bmax.y &&
            v.z >= self.bmin.z && v.z <= self.bmax.z
    }

    #[allow(dead_code)]
    pub fn square(&self) -> f32 {
        let a = (self.bmin.x - self.bmax.x).abs();
        let b = (self.bmin.y - self.bmax.y).abs();
        let c = (self.bmin.z - self.bmax.z).abs();
        (a * b + b * c + a * c) * 2.0
    }
    pub fn vmin(v1 : &Vec3<f32>, v2 : &Vec3<f32>) -> Vec3<f32> {
        let x = if v1.x < v2.x { v1.x } else { v2.x };
        let y = if v1.y < v2.y { v1.y } else { v2.y };
        let z = if v1.z < v2.z { v1.z } else { v2.z };
        Vec3::new(x, y, z)
    }
    pub fn vmax(v1 : &Vec3<f32>, v2 : &Vec3<f32>) -> Vec3<f32> {
        let x = if v1.x > v2.x { v1.x } else { v2.x };
        let y = if v1.y > v2.y { v1.y } else { v2.y };
        let z = if v1.z > v2.z { v1.z } else { v2.z };
        Vec3::new(x, y, z)
    }
}
