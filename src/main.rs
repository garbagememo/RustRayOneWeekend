mod raymod;
use raymod::*;

use rayon::prelude::*;

use std::io::Write;

fn ray_color(r: &Ray,world:&dyn Shape,depth:i64) -> Vec3 {
	if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    let hit_info=world.hit(&r,EPS,f64::MAX);
    if let Some(hit)=hit_info {
        let target =hit.p+hit.n+Vec3::random_hemisphere();
        ray_color(&Ray::new(hit.p,target-hit.p),world,depth-1)*0.5
    } else {
        let t=0.5*(r.d.norm().y+1.0);
        Vec3::new(1.0,1.0,1.0)*(1.0-t)+Vec3::new(0.5,0.7,1.0)*t
    }
}
        

fn main() {

    let args = parameters();
    println!("{:?}", args);
    
    let aspect=16.0/9.0;
    let w: usize = 384;
    let h: usize = ((w as f64)/aspect) as usize;
    let samps:usize = 128;

    let mut image = vec![Color::zero(); (w * h) as usize];

    let v_h=2.0;
    let v_w=aspect*v_h;
    let f_l=1.0;
    let origin=Vec3::new(0.0,0.0,0.0);
    let horizontal=Vec3::new(v_w,0.0,0.0);
    let vertical=Vec3::new(0.0,v_h,0.0);
    let luc=origin-horizontal/2.0+vertical/2.0-Vec3::new(0.0,0.0,f_l);
	let MAX_DEPTH:i64=32;

    let mut world = ShapeList::new();
    world.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));


    let bands: Vec<(usize, &mut [Color])> = image.chunks_mut(w as usize).enumerate().collect();
    bands.into_par_iter().for_each(|(y, band)| {
        for x in 0..w {
            let mut r = Vec3::new(0.0,0.0,0.0);
            for spp in 0..samps {
                let u=(x as f64 + random() ) /(w as f64);
                let v=(y as f64 + random() ) /(h as f64);
                
                let ray=Ray::new(origin,luc+horizontal*u-vertical*v-origin);
                r = r +ray_color(&ray,&world,MAX_DEPTH)/(samps as f64);
            }
            band[x as usize] = r; 
        }
        if (y % 20)==0 {
            print!("y={0}  :",y);
            println!("col={:?}",band[0]);
        };
    });

    //    save_ppm_file("image.ppm", image, w, h);
    save_png_file(&args.output, image, w, h);
}
