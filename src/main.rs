#![allow(unused_imports)]
#![allow(dead_code)]
use rand::Rng;

#[macro_use]
extern crate bmp;
use bmp::{Image, Pixel};

mod fillings;
use fillings::{t1, t2, t3, t4, t5, t6, t7, t8};

fn main() {
    let mut a: Vec<Vec<u16>> = vec![vec![0; 24]; 23];
    let mut idx = 1;
    let _ = t1(0, 0, &mut a, idx);
    idx+=1;
    let _ = t1(5, 0, &mut a, idx);
    idx+=1;
    let _ = t8(11, 0, &mut a, idx);

    // draw the board
    draw(&a, "t");
}

fn draw(a: &[Vec<u16>], name: &str) {
    let x_max = a.len() as u32;
    let y_max = a[0].len() as u32;
    let mut rng = rand::thread_rng();
    let mut colors = vec![(0, 0, 0); 1_000];
    for c in colors.iter_mut() {
        let r = rng.gen_range(0..=255);
        let g = rng.gen_range(0..=255);
        let b = rng.gen_range(0..=255);
        *c = (r, g, b);
    }
    colors[0] = (255, 255, 255);

    // let mut img = Image::new(y_max, x_max);
    let mut img = Image::new(x_max, y_max);
    for x in 0..x_max {
        for y in 0..y_max {
            let (r, g, b) = colors[a[x as usize][y as usize] as usize];
            // img.set_pixel(y, x, px!(r, g, b));
            img.set_pixel(x, y, px!(r, g, b));
        }
    }
    let id = rng.gen_range(0..1000);
    let _ = img.save(format!("{name}-{id}.bmp"));
}
