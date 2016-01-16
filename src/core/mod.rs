pub mod axis;
pub mod ray;
pub mod intersect;
pub mod boundable;
pub mod projectable;

pub use na::*;
pub use std::fmt::Debug;
pub use scene::bbox::*;
pub use core::axis::*;
pub use core::projectable::*;
pub use core::intersect::*;
pub use core::ray::*;
pub use core::boundable::*;

pub trait RayLover : Intersectable + Projectable {}

pub type BoxedObject = Box<RayLover + Send + Sync>;

