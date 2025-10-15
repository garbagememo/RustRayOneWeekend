mod raymod;
use raymod::*;

use rayon::prelude::*;

use std::io::Write;

fn color(ray: &Ray) -> Color {
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

    let bands: Vec<(usize, &mut [Color])> = image.chunks_mut(w as usize).enumerate().collect();
    bands.into_par_iter().for_each(|(y, band)| {
      for x in 0..w {
          let mut r = Vec3::zero();
          r.x= x as f64/255.0;
          r.y= y as f64/255.0;
          r.z= 64.0/255.0;
          band[x as usize] = r ;
      }
    });

    //    save_ppm_file("image.ppm", image, w, h);
    save_png_file(&args.output, image, w, h);
}
