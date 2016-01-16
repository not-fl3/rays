use core::*;

#[derive(Clone, Debug, Copy)]
pub struct Ray {
    pub pos      : Vec3<f32>,
    pub dir      : Vec3<f32>
}

impl Ray {
    pub fn reflect(&self, intersect_pos : Vec3<f32>, intersect_norm : Vec3<f32>) -> Ray{
        Ray {
            pos : intersect_pos,
            dir : self.dir - intersect_norm * (self.dir.dot(&intersect_norm) * 2.0)
        }
    }
}


#[test]
fn test_reflect() {
    let ray = Ray {
        pos : Vec3::new(10.0, 0.0, 0.0),
        dir :  Vec3::new(1.0, 0.0, 0.0)
    };
    assert_eq!(ray.reflect(Vec3::new(10.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)).dir,
               Vec3::new(-1.0, 0.0, 0.0));


}
