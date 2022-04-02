use crate::fillings::{try_fill, unfill};
use crate::{Pos, TMino, drawing};


pub fn get_gap_index(pos: Pos, a: &[Vec<bool>]) -> Option<Pos> {
    let x = pos.x;
    let y = pos.y;

    // First check till end of line
    for (x_pos, col) in a.iter().enumerate().skip(x) {
        if !col[y] {
            return Some(Pos::new(x_pos, y));
        }
    }

    // Then check each row
    let y_max = a[0].len();
    for y_pos in (y+1)..y_max {
        for (x_pos, col) in a.iter().enumerate() {
            if !col[y_pos] {
                return Some(Pos::new(x_pos, y_pos));
            }
        }
    }

    None
}

const MIN_CELLS_X: usize = 3; // 4 for 7mino
const MIN_CELLS_Y: usize = 5; // idk
pub fn load_gap_optims(n_max: usize) -> Vec<bool> {
    // Due to the recursion of finding gaps, at each row the gaps shrink by at least 2
    // => we can be sure all test cases fit
    let mut stack: Vec<(TMino, Pos)> = Vec::with_capacity(100);
    let mut gap_mm = vec![true; n_max];

    for (n, result) in gap_mm.iter_mut().enumerate().skip(1) {
        // Setup a new cells grid
        let x_cells = 2 * MIN_CELLS_X + n;
        let y_cells = MIN_CELLS_Y + 1 + (n / 2);
        let mut cells = vec![vec![false; y_cells]; x_cells];
        cells.iter_mut().take(MIN_CELLS_X).for_each(|col| col[0] = true);
        cells.iter_mut().skip(MIN_CELLS_X + n).for_each(|col| col[0] = true);
        stack.clear();

        let res = gap_optim(&mut cells, &mut stack, n);
        *result = res;
        // *result = gap_optim(&mut cells, &mut stack);
        println!("Result for {n} = {res}");
    }

    gap_mm
}

fn gap_optim(cells: &mut [Vec<bool>], stack: &mut Vec<(TMino, Pos)>, n: usize) -> bool {
    // start backtracking:
    let init_pos = get_first_gap(0, cells).expect("No gap found");
    stack.push((TMino::T0, init_pos));
    'outer: loop {
        let (tmino, pos) = stack.pop().unwrap();

        for curr in tmino.iter() {
            if !try_fill(pos, curr, cells) { continue; }
            stack.push((curr, pos));

            // match get_first_gap(pos.y, cells) {
            match get_first_gap_deep(pos.y, cells) {
                Some(gap_index) => stack.push((TMino::T0, gap_index)),
                // None => return true,
                None => { drawing::draw_optim(stack, cells, format!("optim-{n:02}").as_str()); return true; },
            }
            continue 'outer;
        }

        match stack.last() {
            Some(&(tmino, pos)) => unfill(pos, tmino, cells),
            None => return false,
        }
    }
}

fn get_first_gap_deep(y_index: usize, cells: &[Vec<bool>]) -> Option<Pos> {
    // TODO: Increase to 6 for tmino(7)
    for y_idx in y_index..=(y_index+5) {
        match get_first_gap(y_idx, cells) {
            Some(gap) => return Some(gap),
            None => continue,
        };
    }
    None
    // get_first_gap(y_index, cells).or_else(|| get_first_gap(y_index+1, cells))
}

fn get_first_gap(y_index: usize, cells: &[Vec<bool>]) -> Option<Pos> {
    get_first_nongap_index(Pos::new(0, y_index), cells)
    .and_then(|non_gap|
        get_first_gap_index(non_gap, cells)
    )
    .and_then(|gap_start|
        get_first_nongap_index(gap_start, cells).map(|_| gap_start)
    )
}

/// Searches for the first 0/false after many 1-s/true-s
fn get_first_gap_index(pos: Pos, cells: &[Vec<bool>]) -> Option<Pos> {
    let x = pos.x;
    let y = pos.y;
    if y >= cells[0].len() { return None; }

    for (x_pos, col) in cells.iter().enumerate().skip(x) {
        if !col[y] {
            return Some(Pos::new(x_pos, y));
        }
    }

    None
}

/// Searches for the first 1/true after many 0-s/false-s
fn get_first_nongap_index(pos: Pos, cells: &[Vec<bool>]) -> Option<Pos> {
    let x = pos.x;
    let y = pos.y;
    if y >= cells[0].len() { return None; }

    for (x_pos, col) in cells.iter().enumerate().skip(x) {
        if col[y] {
            return Some(Pos::new(x_pos, y));
        }
    }

    None
}
