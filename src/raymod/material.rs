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

pub struct Dielectric {
    pub ri: f64,
}

impl Dielectric {
    pub const fn new(ri: f64) -> Self {
        Self { ri }
    }
    pub fn schlick(cosine:f64,ri:f64) -> f64 {
        let r0=((1.0-ri)/(1.0+ri)).powi(2);
        r0+(1.0-r0)*(1.0-cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let reflected = ray.d.reflect(hit.n);
        let (outward_normal, ni_over_nt,cosine) = {
            let dot =  ray.d.dot(&hit.n);
            if dot > 0.0 {
                (hit.n*-1.0, self.ri,self.ri * dot / ray.d.length().sqrt() )
            } else {
                (hit.n, 1.0/self.ri ,self.ri * dot / ray.d.length().sqrt()*-1.0  )
            }
        };
        if let Some(refracted) = (ray.d*-1.0).refract(outward_normal, ni_over_nt) {
            if Vec3::random_full().x > Self::schlick(cosine,self.ri) {
                return Some(ScatterInfo::new(Ray::new(hit.p, refracted),
                                             Vec3::new(1.0,1.0,1.0)))
            } 
            
        }
        Some(ScatterInfo::new(Ray::new(hit.p, reflected), Vec3::new(1.0,1.0,1.0)))
    }
}


