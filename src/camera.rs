use na::*;
use core::ray::*;

pub struct Camera {
    pub pos     : Vec3<f32>,
    pub look_at : Vec3<f32>,
    pub up      : Vec3<f32>,
    pub fov     : f32,
    pub w       : i32,
    pub h       : i32
}

impl Camera {
    pub fn get_ray(&self, x : i32, y : i32) -> Ray {
        let dir = (self.look_at - self.pos).normalize();
        let right = (dir.cross(&self.up)).normalize();
        let dx = (x - self.w / 2) as f32;
        let dy = (y - self.h / 2) as f32;
        Ray {
            pos : self.pos,
            dir : (dir - self.up * dy * 0.006 + right * dx * 0.006).normalize()
        }
    }

}


#[test]
fn test_camera() {
    let camera = Camera {
        pos     : Vec3::new(-10.0, 0.0, 0.0),
        look_at : Vec3::new(0.0, 0.0, 0.0),
        up      : Vec3::new(0.0, 1.0, 0.0),
        fov     : 60.0,
        w       : 300,
        h       : 200
    };
    assert_eq!(camera.get_ray(150, 100).pos, camera.pos);
    assert_eq!(camera.get_ray(150, 100).dir, Vec3::new(1.0, 0.0, 0.0));
    assert!(camera.get_ray(100, 100).dir.z < 0.0);
    assert!(camera.get_ray(200, 100).dir.z > 0.0);
    assert!(camera.get_ray(150, 10).dir.y > 0.0);
    assert!(camera.get_ray(150, 190).dir.y < 0.0);
}
