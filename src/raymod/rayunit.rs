use crate::raymod::*;

use std::sync::Arc;
use std::f64::consts::*;


#[derive(Debug)]
pub struct Ray {
    pub o: Vec3,
    pub d: Vec3,
}

impl Ray {
    pub fn new(o: Vec3, d: Vec3) -> Ray {
        Ray { o, d }
    }
    pub fn at(&self,t:f64)->Vec3{
        self.o + self.d*t
    }
}

pub enum Refl {
    Diff,
    Spec,
    Refr,
}

pub struct HitInfo {
    pub t: f64,
    pub p: Vec3,
    pub n: Vec3,
    pub m: Arc <dyn Material>,
}

impl HitInfo {
    pub fn new(t:f64,p:Vec3,n:Vec3,m: Arc <dyn Material>)->Self {
        Self{t,p,n,m}
    }
}

pub trait Shape: Sync {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) ->Option<HitInfo>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material:Arc<dyn Material> 
}

impl Sphere {
    pub const fn new(center: Vec3, radius: f64,material:Arc<dyn Material>) -> Self {
        Self { center, radius ,material}
    }
}

impl Shape for Sphere {
    fn hit(&self, r: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
        let oc = r.o - self.center;
        let a = r.d.dot(&r.d);
        let b = r.d.dot(&oc)*2.0;
        let c = oc.dot(&oc) - self.radius * self.radius;
        let d = b*b-4.0*a*c;

        if d > 0.0 {
            let root = d.sqrt();
            let temp = (-b - root) / (2.0*a);
            if temp < t1 && temp > t0 {
                let p=r.at(temp);
                return Some(HitInfo::new(temp, p, (p- self.center)/self.radius, Arc::clone(&self.material)) );
            }
            let temp = (-b + root) / (2.0*a);
            if temp < t1 && temp > t0 {
                let p=r.at(temp);
                return Some(HitInfo::new(temp,p,(p- self.center)/self.radius,Arc::clone(&self.material)) );
            }
        }
        None
    }
}

pub struct ShapeList {
    pub objects: Vec<Box<dyn Shape>>,
}

impl ShapeList {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }
    pub fn push(&mut self, object: Box<dyn Shape>) {
        self.objects.push(object);
    }
}

impl Shape for ShapeList {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
        let mut hit_info:Option<HitInfo> = None;
        let mut closest_so_far = t1;
        for object in &self.objects {
            if let Some(info) = object.hit(ray, t0, closest_so_far) {
                closest_so_far = info.t;
                hit_info = Some(info);
            }
        }
        hit_info
    }
}
