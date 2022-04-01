#![allow(unused_imports)]
#![allow(dead_code)]
#[macro_use]
extern crate bmp;

mod tmino;
mod fillings;
mod gap_finder;
mod drawing;

use tmino::{TMino, Pos};
use fillings::{try_fill, unfill};
use gap_finder::get_gap_index;
use drawing::draw;

pub const X_MAX: usize = 23;
pub const Y_MAX: usize = 24;

fn main() {
    let mut cells = vec![vec![false; Y_MAX]; X_MAX];
    let mut stack: Vec<(TMino, Pos)> = Vec::with_capacity(100);
    // LOAD invalid gaps

    stack.push((TMino::T0, Pos::new(0, 0)));
    'outer:
    loop {
        let (tmino, pos) = stack.pop().expect("Impossible.. like fr");

        for curr in tmino.iter() {
            if !try_fill(pos, curr, &mut cells) { continue; }
            stack.push((curr, pos));

            if let Some(gap_index) = get_gap_index(pos, &cells) {
                stack.push((TMino::T0, gap_index));
                continue 'outer;
            }
            else {
                draw(&stack, "solution");
                break 'outer;
            }
        }

        let &(tmino, pos) = stack.last().expect("Impossible 2");
        unfill(pos, tmino, &mut cells);
    }

    println!("Done! Filled!");
}
