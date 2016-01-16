use std::cmp::Ordering;
use std::cmp::Ordering::*;
use std::f32::MAX;

use core::*;

use objects::mesh::*;
use objects::sphere::*;

use materials::*;

pub struct KDTreeRoot {
    objects : Vec<BoxedObject>,
    tree    : Box<KDTreeNode>
}

pub struct KDTreeNode<> {
    bbox    : BBox,
    indices : Vec<i32>,
    left    : Option<Box<KDTreeNode>>,
    right   : Option<Box<KDTreeNode>>
}

impl KDTreeNode {
    pub fn intersect_node(&self, r : Ray) -> Option<f32> {
            self.bbox.intersect(r)
    }

    fn objects<'a>(&self, data : &'a Vec<BoxedObject>) -> Vec<&'a BoxedObject> {
        self.indices
            .iter()
            .map(|i| { &data[*i as usize] })
            .collect::<Vec<&BoxedObject>>()

    }
    fn intersect(&self, objects : &Vec<BoxedObject>, r : Ray) -> Option<(f32, IntersectData)> {
        match (&self.left, &self.right) {
            (&None, &None) => match self.intersect_node(r) {
                Some(_) => self.objects(objects).intersect(r),
                None => None
            },
            (&None, &Some(ref x)) | (&Some(ref x), &None) => x.intersect(&objects, r),
            (&Some(ref x), &Some(ref y)) => match (x.intersect_node(r), y.intersect_node(r)) {
                (None, None) => None,
                (None, Some(_)) => y.intersect(objects, r),
                (Some(_), None) => x.intersect(objects, r),
                (Some(lx), Some(ly)) if lx > ly => y.intersect_or_next(x, objects, r),
                (Some(_), Some(_))  => x.intersect_or_next(y, objects, r),
            }
        }
    }
    fn intersect_or_next(&self, next : &KDTreeNode, objects : &Vec<BoxedObject>, r : Ray) -> Option<(f32, IntersectData)>{
        match self.intersect(objects, r.clone()) {
            res @ Some(_) => res,
            None => next.intersect(objects, r)
        }
    }
}

impl RayLover for KDTreeRoot {}
impl Projectable for KDTreeRoot {}
impl Intersectable for KDTreeRoot {
    fn intersect(&self, r: Ray) -> Option<(f32, IntersectData)> {
        return self.tree.intersect(&self.objects, r)
    }
}

fn compare_objects(axis : &Axis, a : &BoxedObject, b : &BoxedObject) -> Ordering {
    axis.get(a.get_min_bound()).partial_cmp(
        &axis.get(b.get_min_bound())).unwrap_or(Less)
}

#[allow(dead_code)]
fn split_with_sort(bbox : BBox, indcs : &Vec<i32>, objects : &Vec<BoxedObject>) -> ((BBox, Vec<i32>), (BBox, Vec<i32>)) {
    let mut indcs = indcs.clone();
    let mut min_sah = MAX;
    let mut min_axis = Axis::X;
    let mut min_split = 0.0;

    for axis in &[Axis::X, Axis::Y, Axis::Z] {
        indcs.sort_by(|a, b| {compare_objects(&axis, &objects[*a as usize], &objects[*b as usize])});

        for (i, c) in (1 .. indcs.len()).zip(1..) {
            const CONST_EMPTY : f32 = 0.1;

            let split = axis.get(objects[i].get_min_bound());
            if split == axis.get(bbox.bmin) {
                continue;
            }

            let square_left  = bbox.split_left(split, *axis).square();
            let square_right = bbox.split_right(split, *axis).square();
            let count_left = c as f32;
            let count_right = (indcs.len() - c) as f32;
            let sah = CONST_EMPTY + square_left * count_left + square_right * count_right;
            if min_sah > sah {
                // println!("sah: {} num: {} split: {} l: {} r : {} sl : {} sr : {}", sah, c, split, count_left, count_right, square_left, square_right);
                min_sah = sah;
                min_axis = axis.clone();
                min_split = split;
            }
        }
    }

    let bbox_left = bbox.split_left(min_split, min_axis);
    let bbox_right = bbox.split_right(min_split, min_axis);
    let mut lefts = Vec::new();
    let mut rights = Vec::new();
    for i in &indcs {
        //println!("{:?}", bbox_left);
        if objects[*i as usize].in_box(bbox_left) {
            lefts.push(*i);
        }
        //println!("{:?}", bbox_right);
        if objects[*i as usize].in_box(bbox_right) {
            rights.push(*i);
        }
    }

    //println!("{} {} {} {} {:?} {:?} {:?}", indcs.len(), lefts.len(), rights.len(), min_split, bbox, bbox_left, bbox_right);

    assert!(bbox_left != bbox);
    assert!(bbox_right != bbox);
    ((bbox_left, lefts),
     (bbox_right, rights))
}

fn split(bbox : BBox, indcs : &Vec<i32>, objects : &Vec<BoxedObject>) -> ((BBox, Vec<i32>), (BBox, Vec<i32>)) {
    split_easy(bbox, indcs, objects)
}

fn split_easy(bbox : BBox, indcs : &Vec<i32>, objects : &Vec<BoxedObject>) -> ((BBox, Vec<i32>), (BBox, Vec<i32>)) {
    let indcs = indcs.clone();

    let x = (bbox.bmin.x - bbox.bmax.x).abs();
    let y = (bbox.bmin.y - bbox.bmax.y).abs();
    let z = (bbox.bmin.z - bbox.bmax.z).abs();
    let axis = match (x, y, z) {
        (x, y, z) if x > y && x > z => Axis::X,
        (x, y, z) if y >= x && y > z =>  Axis::Y,
        _ => Axis::Z
        };

    let split = axis.get(bbox.bmin) + (axis.get(bbox.bmax) - axis.get(bbox.bmin)) / 2.0;
    let bbox_left  = bbox.split_left(split, axis);
    let bbox_right  = bbox.split_right(split, axis);

    let mut lefts = Vec::new();
    let mut rights = Vec::new();
    for i in &indcs {
        match (objects[*i as usize].in_box(bbox_left), objects[*i as usize].in_box(bbox_right)) {
            (true, true)  => {
                lefts.push(*i);
                rights.push(*i);
            }
            (true, false) => lefts.push(*i),
            (false, true) => rights.push(*i),
            _ => panic!("hobo triangle found")
        }
    }

    ((bbox_left, lefts),
     (bbox_right, rights))
}

#[allow(dead_code)]
fn gen_test_spheres() -> (Vec<BoxedObject>, BBox) {
    let spheres = vec![
        Box::new(Sphere {
            pos : Vec3::new(2.5, 2.5, 2.5),
            radius : 5.0,
            material : test_mat()}) as BoxedObject,
        Box::new(Sphere {
            pos : Vec3::new(8.5, 2.5, 2.5),
            radius : 5.0,
            material : test_mat()})];
    let bbox = spheres.get_bound_box();
    (spheres, bbox)
}
#[test]
fn test_bbox_sphere() {
    let (_, bbox) = gen_test_spheres();
    assert_eq!(bbox.bmin, Vec3::new(0.0, 0.0, 0.0));
    assert_eq!(bbox.bmax, Vec3::new(11.0, 5.0, 5.0));
}

#[test]
fn test_split_sphere() {
    let (spheres, bbox) = gen_test_spheres();
    let ((_, l), (_, r)) = split(bbox, &vec![0, 1], &spheres);
    assert_eq!(l.len(), 1);
    assert_eq!(r.len(), 1);
}

fn build_node(bbox : BBox, indcs : Vec<i32>, objects : &Vec<BoxedObject>) -> KDTreeNode {
    const MAX_INDCS : usize = 30;

    if indcs.len() <  MAX_INDCS {
        KDTreeNode {
            bbox    : bbox,
            indices : indcs,
            left    : None,
            right   : None
        }
    } else {
        let ((bl, l), (br, r)) = split(bbox, &indcs, objects);
        KDTreeNode {
            bbox    : bbox,
            indices : indcs,
            left    : Some(Box::new(build_node(bl, l, objects))),
            right   : Some(Box::new(build_node(br, r, objects)))
        }
    }
}

pub fn build_kd_tree(objects: Vec<BoxedObject>) -> KDTreeRoot {
    KDTreeRoot {
        tree    : Box::new(build_node(
            objects.get_bound_box(),
            (0 .. objects.len() as i32).collect(),
            &objects)),
        objects : objects

    }
}

#[test]
fn test_kd_three_cow() {
    let cow = load_mesh("models/chair/chair.obj",
                        Vec3::new(0.0, 9.0, 0.0),
                        Vec3::new(2.5, 2.5, 2.5),
                        &Rot3::new_identity(3),
                        test_mat()).unwrap();

    let kd_three = build_kd_tree(cow);
    let bbox = kd_three.tree.bbox;
    assert!(bbox.bmin.x >= -15.0 && bbox.bmin.x <= 15.0);
    assert!(bbox.bmax.x >= -20.0 && bbox.bmax.x <= 20.0, "!! {}", bbox.bmax.x);
    assert!(bbox.bmin.x < bbox.bmax.x);
    assert!(bbox.bmin.y < bbox.bmax.y);
    assert!(bbox.bmin.z < bbox.bmax.z);
}

