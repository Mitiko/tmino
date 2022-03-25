#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::VecDeque;

use rand::Rng;

#[macro_use]
extern crate bmp;
use bmp::{Image, Pixel};

mod fillings;
use fillings::{t1, t2, t3, t4, t5, t6, t7, t8, get_gap_index};
use fillings::{u1, u2, u3, u4, u5, u6, u7, u8};

pub const X_MAX: usize = 23;
pub const Y_MAX: usize = 24;

#[derive(Debug, Clone, Copy)]
enum TMino {
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

    #[must_use]
    fn apply(&self, pos: Pos, a: &mut [Vec<u16>], idx: u16) -> bool {
        match self {
            TMino::T0 => false,
            TMino::T1 => t1(pos.x, pos.y, a, idx),
            TMino::T2 => t2(pos.x, pos.y, a, idx),
            TMino::T3 => t3(pos.x, pos.y, a, idx),
            TMino::T4 => t4(pos.x, pos.y, a, idx),
            TMino::T5 => t5(pos.x, pos.y, a, idx),
            TMino::T6 => t6(pos.x, pos.y, a, idx),
            TMino::T7 => t7(pos.x, pos.y, a, idx),
            TMino::T8 => t8(pos.x, pos.y, a, idx)
        }
    }

    fn unapply(&self, pos: Pos, a: &mut [Vec<u16>]) {
        match self {
            TMino::T0 => { },
            TMino::T1 => u1(pos.x, pos.y, a),
            TMino::T2 => u2(pos.x, pos.y, a),
            TMino::T3 => u3(pos.x, pos.y, a),
            TMino::T4 => u4(pos.x, pos.y, a),
            TMino::T5 => u5(pos.x, pos.y, a),
            TMino::T6 => u6(pos.x, pos.y, a),
            TMino::T7 => u7(pos.x, pos.y, a),
            TMino::T8 => u8(pos.x, pos.y, a)
        }
    }
}

fn main() {
    let mut a: Vec<Vec<u16>> = vec![vec![0; Y_MAX]; X_MAX];
    let mut stack: Vec<(TMino, Pos)> = Vec::with_capacity(100);

    let mut i = 0;
    // start backtracking:
    stack.push((TMino::T0, Pos { x: 0, y: 0 }));
    'outer:
    loop {
        // track stack len
        println!("Depth: {}", stack.len());
        if stack.len() >= 88 && i == 0 {
            i += 1;
            draw(&a, "depth-88");
        }
        if stack.len() >= 89 && i == 1 {
            i += 1;
            draw(&a, "depth-89");
        }
        if stack.len() >= 90 && i == 2 {
            i += 1;
            draw(&a, "depth-90");
        }
        if stack.len() >= 91 && i == 3 {
            i += 1;
            draw(&a, "depth-91");
        }
        let top = stack.pop();
        if top.is_none() { println!("Impossible!"); break; }

        let (mut tmino, pos) = top.unwrap();
        while let Some(curr) = tmino.next() {
            tmino = curr;
            let idx = stack.len() as u16 + 1;
            if curr.apply(pos, &mut a, idx) {
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
        tmino.unapply(pos, &mut a);
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
