mod raymod;
use raymod::*;

use rayon::prelude::*;

use std::io::Write;

fn ray_color(ray: &Ray) -> Color {
    let u_d=ray.d.norm();
    let t=0.5*(u_d.y+1.0);
    Vec3::new(1.0,1.0,1.0)*(1.0-t)+Vec3::new(0.5,0.7,1.0)*t
}
        

fn main() {

    let args = parameters();
    println!("{:?}", args);
    
    let aspect=16.0/9.0;
    let w: usize = 384;
    let h: usize = ((w as f64)/aspect) as usize;

    let mut image = vec![Color::zero(); (w * h) as usize];

    let v_h=2.0;
    let v_w=aspect*v_h;
    let v_l=1.0;
    let origin=Vec3::new(0.0,0.0,0.0);
    let horizontal=Vec3::new(v_w,0.0,0.0);
    let vertical=Vec3::new(0.0,v_h,0.0);
    let llc=origin-horizontal/2.0-vertical/2.0-Vec3::new(0.0,0.0,v_l);


    let bands: Vec<(usize, &mut [Color])> = image.chunks_mut(w as usize).enumerate().collect();
    bands.into_par_iter().for_each(|(y, band)| {
      for x in 0..w {
          let u=x as f64/(w as f64);
          let v=y as f64/(h as f64);
          
          let ray=Ray::new(origin,llc+horizontal*u+vertical*v-origin);
          let col=ray_color(&ray);
          println!("{:?}",col);
          band[x as usize] = col;
      }
    });

    //    save_ppm_file("image.ppm", image, w, h);
    save_png_file(&args.output, image, w, h);
}
