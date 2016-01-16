pub mod bbox;
pub mod kdtree;

use core::*;
use camera::*;
use light::*;
use objects::sphere::*;
use objects::plane::*;
use objects::mesh::*;
use materials::*;
use scene::kdtree::*;

pub struct Scene {
    pub cam     : Camera,
    pub objects : Vec<BoxedObject>,
    pub lights  : Vec<Light>
}

fn obj_min(min : Option<(f32, IntersectData)>, obj : &BoxedObject, r : Ray) -> Option<(f32, IntersectData)>{
    let cur = (*obj).intersect(r);
    match min {
        None => cur,
        Some((min_distance, _)) => {
            match cur {
                None => min,
                Some(cur_distance) =>
                    if cur_distance.0 > min_distance {
                        min
                    } else {
                        Some(cur_distance)
                    }
            }
        }
    }
}

impl Intersectable for Vec<BoxedObject> {
    fn intersect(&self, r: Ray) -> Option<(f32, IntersectData)> {
        let iter = self.iter();
        iter.fold(None, |acc, cur| { obj_min(acc, cur, r) })
    }
}

impl<'a> Intersectable for Vec<&'a BoxedObject> {
    fn intersect(&self, r: Ray) -> Option<(f32, IntersectData)> {
        let iter = self.iter();
        iter.fold(None, |acc, cur| { obj_min(acc, *cur, r) })
    }
}


#[allow(dead_code)]
pub fn some_cow_scene(w : i32, h : i32) -> Scene {
    let camera = Camera {
        pos     : Vec3::new(-20.0, 8.1, -3.0),
        look_at : Vec3::new(0.0, 0.0, 0.0),
        up      : Vec3::new(0.0, 1.0, 0.0),
        fov     : 60.0,
        w       : w,
        h       : h
    };
    let cow = load_mesh("cow.obj",
                        Vec3::new(0.0, 9.0, 0.0),
                        Vec3::new(2.5, 2.5, 2.5),
                        &Rot3::new(Vec3::new(0.0, 3.1415, 0.0)),
                        Box::new(SolidColorMat {
                            color : Vec3::new(0.6, 0.1, 0.6)
                        })).unwrap();
    let objects = vec![
        Box::new(Sphere
                 {
                     pos      : Vec3::new(0.0, 8.0, -17.0),
                     radius   : 8.0,
                     material : Box::new(SolidColorMat { color : Vec3::new(1.0, 0.0, 0.0) })
                 }) as BoxedObject,
        // Box::new(Sphere
        //          {
        //              pos      : Vec3::new(0.0, 8.0, 0.0),
        //              radius   : 8.0,
        //              material : Material::SolidColor { color : Vec3::new(0.0, 1.0, 0.0) }
        //          }) as Box<Intersectable>,
        Box::new(Sphere
                 {
                     pos      : Vec3::new(0.0, 8.0, 17.0),
                     radius   : 8.0,
                     material : Box::new(SolidColorMat { color : Vec3::new(0.0, 0.0, 1.0) })
                 }),
        Box::new(Plane
                 {
                     pos    : Vec3::new(0.0, 0.0, 0.0),
                     normal : Vec3::new(0.0, 1.0, 0.0),
                     material : Box::new(ChessMat{sx : 0.5, sy : 0.5})
                 }),
        Box::new(build_kd_tree(cow))
            ];
    Scene {
        cam : camera,
        objects : objects,
        lights : vec![Light::PointLight(Vec3::new(-10.0, 3.0, 10.0), Vec3::new(0.6, 0.0, 0.0)),
                      Light::PointLight(Vec3::new(-10.0, 3.0, -10.0), Vec3::new(0.0, 0.0, 0.6)),
                      Light::PointLight(Vec3::new(-10.0, 3.0, 0.0), Vec3::new(0.0, 0.6, 0.0))]}
}

#[allow(dead_code)]
pub fn some_room_scene(w : i32, h : i32) -> Scene {
    let camera = Camera {
        pos     : Vec3::new(-10.0, 8.1, -3.0),
        look_at : Vec3::new(0.0, 0.0, 10.0),
        up      : Vec3::new(0.0, 1.0, 0.0),
        fov     : 60.0,
        w       : w,
        h       : h
    };

    let objects = vec![
        Box::new(Plane
                 {
                     pos      : Vec3::new(0.0, 0.0, 0.0),
                     normal   : Vec3::new(0.0, 1.0, 0.0),
                     material : Box::new(TextureMat::load("models/lin.jpg", 0.2))
//                      material : Box::new(ChessMat { sx : 0.5, sy : 0.5 })
                 }) as BoxedObject,
        Box::new(Sphere
                 {
                     pos      : Vec3::new(-4.0, 1.0, 2.0),
                     radius   : 1.0,
                     material : Box::new(SolidColorMat { color : Vec3::new(1.0, 0.0, 0.0) })
                 }),
        // Box::new(build_kd_tree(load_mesh("models/holder/holder.obj",
        //                                  Vec3::new(0.0, 0.0, 17.0),
        //                                  Vec3::new(0.03, 0.03, 0.03),
        //                                  &Rot3::new(Vec3::new(0.0, 0.0, 0.0)),
        //                                  Box::new(TextureMat::load("models/holder/wood.jpg"))).unwrap())),
        Box::new(build_kd_tree(load_mesh("models/chair/chair.obj",
                                         Vec3::new(0.0, 0.0, 0.0),
                                         Vec3::new(7.0, 7.0, 7.0),
                                         &Rot3::new(Vec3::new(0.0, 0.0, 0.0)),
                                         Box::new(TextureMat::load("models/chair/chair_d.png", 0.01))).unwrap())),
            ];
    Scene {
        cam : camera,
        objects : objects,
        lights : vec![Light::PointLight(Vec3::new(-15.0, 10.0, 4.0), Vec3::new(1.0, 1.0, 1.0))]}
}

#[allow(dead_code)]
pub fn some_easy_scene(w : i32, h : i32) -> Scene {
    let camera = Camera {
        pos     : Vec3::new(0.0, 20.0, 0.0),
        look_at : Vec3::new(0.0, 0.0, 0.0),
        up      : Vec3::new(1.0, 0.0, 0.0),
        fov     : 60.0,
        w       : w,
        h       : h
    };
    let scene = vec![
        Box::new(Sphere
                 {
                     pos      : Vec3::new(0.0, 3.0, 0.0),
                     radius   : 3.0,
                     material : Box::new(SolidColorMat { color : Vec3::new(1.0, 0.0, 0.0) })
                 }) as BoxedObject,
        Box::new(Plane
                 {
                     pos    : Vec3::new(0.0, 0.0, 0.0),
                     normal : Vec3::new(0.0, 1.0, 0.0),
                     material : Box::new(ChessMat { sx : 0.01, sy : 0.01 })
                 })];
    Scene {cam : camera, objects : scene, lights : vec![
        point_light_white(Vec3::new(50.0, 8.0, 0.0)),
        point_light_white(Vec3::new(-50.0, 8.0, 0.0)),
        point_light_white(Vec3::new(0.0, 8.0, 50.0)),
        point_light_white(Vec3::new(0.0, 8.0, -50.0))]}
}





