extern crate image;
extern crate rand;
extern crate nalgebra as na;

mod core;
mod light;
mod materials;
mod objects;
mod camera;
mod scene;

use core::*;

use image::*;
use scene::*;
use light::*;

use std::thread;
use std::sync::{Arc, Mutex};
use rand::*;

const W : i32 = 800;
const H : i32 = 600;
const CHUNK_SIZE : i32 = 5000;
const WORKERS_COUNT : i32 = 4;

fn random_point() -> Vec3<f32> {
    let u = random::<f32>();
    let v = random::<f32>();
    let angle1 =  u * 2.0 * 3.1415926535;
    let angle2 = (v * 2.0 - 1.0).cos().powf(-1.0);
    let x = angle1.sin() * angle2.cos();
    let y = angle1.sin() * angle1.sin();
    let z = angle1.cos();
    Vec3::new(x, y, z)

}
fn soft_shadow(scene : &Scene, pos : Vec3<f32>, light_pos : &Vec3<f32>) -> f32 {
    let light_pos = *light_pos + random_point() * 0.3;
    let light_dir = (light_pos - pos).normalize();
    let light_dist = (light_pos - pos).norm();
    let mut shadow = 1.0;
    for _ in 1 .. 20 {
        let shadow_inter = scene.objects.intersect(Ray {
            pos : pos,
            dir : light_dir});
        match shadow_inter {
            Some((sl, _)) if sl < light_dist => shadow *= 0.93,
            _ => {}
        }
    }
    shadow
}
fn aliased_soft_shadow(scene : &Scene, pos : Vec3<f32>, light_pos : &Vec3<f32>) -> f32 {
    let n = 25;
    let mut sum = 0.0;
    for _ in 1 .. n {
        sum += soft_shadow(scene, pos, light_pos);
    }
    sum / (n as f32)
}
fn color(scene : &Scene, intersect: &IntersectData) -> Vec3<f32> {
    let mat = intersect.material.clone_box();
    let color = scene.lights.iter().map(|l| {
        match *l {
            Light::PointLight(v, c) => {
                let light_dir = (v - intersect.pos).normalize();
                let force = light_dir.dot(&intersect.normal);
                let shadow = aliased_soft_shadow(scene, intersect.pos + intersect.normal * 0.01, &v);
                let color = mat.get_color(&intersect) * c * shadow;
                (force.max(0.0), color)
            }
        }
    }).fold(Vec3::new(0.0, 0.0, 0.0), |acc, (force, color)| {
        acc + color * force
    });

    return color * 255.0;
}

fn shade(scene : &Scene, ray : Ray, depth : i32) -> Vec3<f32> {
    const MAX_DEPTH : i32 = 3;
    match scene.objects.intersect(ray) {
        None => Vec3::new(0.0, 0.0, 0.0),
        Some((_, intersect)) => {
            let refl = ray.reflect(intersect.pos + intersect.normal * 0.01, intersect.normal);
            let color = color(scene, &intersect);
            if depth < MAX_DEPTH {
                color + shade(scene, refl, depth + 1) * intersect.material.get_reflect()
            }
            else {
                color
            }
        }
    }
}
fn render_part(scene : &Scene, start : i32, end : i32) -> Vec<u8> {
    let mut vec = Vec::with_capacity((end - start) as usize * 3);
    for pos in start .. end {
        let x = pos % (W as i32);
        let y = pos / (W as i32);

        let ray = scene.cam.get_ray(x as i32, y as i32);
        let color = shade(scene, ray, 0);
        vec.push(color.x.min(255.0) as u8);
        vec.push(color.y.min(255.0) as u8);
        vec.push(color.z.min(255.0) as u8);
    }
    vec
}

fn render_thread(passed : Arc<Mutex<i32>>, scene : Arc<Scene>) -> Vec<(i32, Vec<u8>)> {
    let mut calced = Vec::new();

    while {
        let passed = {
            let mut passed = passed.lock().unwrap();
            *passed += CHUNK_SIZE;
            *passed
        };
        let start = passed - CHUNK_SIZE;
        if passed <= W * H {
            println!("calced: {} {}", passed / W, passed);
            let points = render_part(&scene, start, min(W * H, start + CHUNK_SIZE));
            calced.push((start, points));
            true
        } else {
            false
        }
    } {}
    calced
}

fn main() {
    let scene = Arc::new(some_room_scene(W as i32, H as i32));
    let passed = Arc::new(Mutex::new(0));
    let mut threads = Vec::new();
    for _ in 0 .. WORKERS_COUNT {
        let scene = scene.clone();
        let passed = passed.clone();
        let thread = thread::spawn(|| {
            render_thread(passed, scene)
        });
        threads.push(thread);
    }
    let mut buf : [u8; (W * H * 3) as usize] = [0; (W * H * 3) as usize];
    for thread in threads {
        for (pos, vec) in thread.join().unwrap() {
            for (i, c) in vec.iter().enumerate() {
                buf[pos as usize * 3 + i] = *c;
            }
        }
    }

    let _ = image::save_buffer("res.png", &buf, W as u32, H as u32, image::RGB(8));
}
