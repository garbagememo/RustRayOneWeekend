use crate::raymod::*;

use std::sync::Arc;
use std::f64::consts::*;


pub struct ScatterInfo {
    pub ray: Ray,
    pub albedo: Color,
}


pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo>;
}


impl ScatterInfo {
    pub fn new(ray: Ray, albedo: Vec3) -> Self {
        Self { ray, albedo }
    }
}
pub struct Lambertian {
    pub albedo: Vec3,
}
impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let target = hit.p + hit.n + Vec3::random_hemisphere();
        Some(ScatterInfo::new(Ray::new(hit.p, target - hit.p), self.albedo))
    }
}

pub struct Metal {
    pub albedo:Vec3,
    pub fuzz:f64,
}

impl Metal {
    pub fn new(albedo: Vec3,fuzz:f64) -> Self {
        Self { albedo , fuzz }
    }
}
impl Material for Metal {
    fn scatter(&self, _ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let mut reflected = _ray.d.norm().reflect(hit.n);
        reflected = reflected + Vec3::random_hemisphere()*self.fuzz;
        if reflected.dot(&hit.n)>0.0 {
            Some(ScatterInfo::new(Ray::new(hit.p, reflected), self.albedo))
        }else{
            None
        }        
    }
}
