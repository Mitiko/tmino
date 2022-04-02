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
use gap_finder::{load_gap_optims, get_gap_index};
use drawing::draw;

fn main() {
    let gap_mm = load_gap_optims(50);
    dbg!(gap_mm[0]);
    std::process::exit(1);

    let x_max = 23;
    let y_max = 24;
    let mut cells = vec![vec![false; y_max]; x_max];
    let mut stack: Vec<(TMino, Pos)> = Vec::with_capacity(100);
    // LOAD invalid gaps

    stack.push((TMino::T0, Pos::new(0, 0)));
    'outer: loop {
        let (tmino, pos) = stack.pop().unwrap();

        for curr in tmino.iter() {
            if !try_fill(pos, curr, &mut cells) { continue; }
            stack.push((curr, pos));

            match get_gap_index(pos, &cells) {
                Some(gap_index) => stack.push((TMino::T0, gap_index)),
                None => draw(&stack, "solution"),
            }
            continue 'outer; // Calling draw terminates the program
        }

        match stack.last() {
            Some(&(tmino, pos)) => unfill(pos, tmino, &mut cells),
            None => break,
        }
    }

    // If solutions are found, a bmp will be generated and the program will exit
    println!("No solutions found!");
}
