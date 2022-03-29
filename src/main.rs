#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::VecDeque;

use rand::Rng;

#[macro_use]
extern crate bmp;
use bmp::{Image, Pixel};

mod fillings;
use fillings::{get_gap_index, try_fill, unfill};

pub const X_MAX: usize = 23;
pub const Y_MAX: usize = 24;

#[derive(Debug, Clone, Copy)]
pub enum TMino {
    T0, T1, T2, T3, T4, T5, T6, T7, T8
}

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    x: usize,
    y: usize
}

impl TMino {
    fn next(self) -> Option<Self> {
        match self {
            TMino::T0 => Some(Self::T1),
            TMino::T1 => Some(Self::T3),
            TMino::T3 => Some(Self::T2),
            TMino::T2 => Some(Self::T4),
            TMino::T4 => Some(Self::T6),
            TMino::T6 => Some(Self::T5),
            TMino::T5 => Some(Self::T8),
            TMino::T8 => Some(Self::T7),
            TMino::T7 => None
        }
    }
}

fn main() {
    let mut a: Vec<Vec<u16>> = vec![vec![0; Y_MAX]; X_MAX];
    let mut stack: Vec<(TMino, Pos)> = Vec::with_capacity(100);

    // start backtracking:
    stack.push((TMino::T0, Pos { x: 0, y: 0 }));
    'outer:
    loop {
        let top = stack.pop();
        if top.is_none() { println!("Impossible!"); break; }

        let (mut tmino, pos) = top.unwrap();
        while let Some(curr) = tmino.next() {
            tmino = curr;
            let idx = stack.len() as u16 + 1;
            if try_fill(pos, &mut a, idx, curr) {
                // if gap index not found, we're done
                // TODO: if t1 or t3, optim search by starting a bit on the right
                if let Some(gap_index) = get_gap_index(pos.x, pos.y, &mut a) {
                    stack.push((curr, pos));
                    stack.push((TMino::T0, gap_index));
                    continue 'outer;
                }
                else {
                    println!("Done! Filled!");
                    draw(&a, "solution");
                    break 'outer;
                }
            }
        }

        let top = stack.last();
        if top.is_none() { println!("Impossible! 2"); break; }
        let &(tmino, pos) = top.unwrap();
        unfill(pos, &mut a, tmino);
    }
}

fn draw(a: &[Vec<u16>], name: &str) {
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
    let mut img = Image::new(X_MAX as u32, Y_MAX as u32);
    for x in 0..X_MAX as u32 {
        for y in 0..Y_MAX as u32 {
            let (r, g, b) = colors[a[x as usize][y as usize] as usize];
            // img.set_pixel(y, x, px!(r, g, b));
            img.set_pixel(x, y, px!(r, g, b));
        }
    }
    let id: u32 = rng.gen_range(0..1000);
    let _ = img.save(format!("{name}-{id}.bmp"));
}
