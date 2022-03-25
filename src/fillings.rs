use crate::{Pos, X_MAX, Y_MAX};

// OR:
// 0 0 -> 0
// 0 1 -> 1
// 1 0 -> 1
// 1 1 -> 1
#[must_use]
pub fn t1(x: usize, y: usize, a: &mut [Vec<u16>], idx: u16) -> bool {
    if x + 4 >= X_MAX || y + 1 >= Y_MAX { return false; }

    let mut state_a = 0;
    let mut state_b = 0;
    state_a |= a[x][y];
    state_a |= a[x+1][y];
    state_a |= a[x+1][y+1];
    state_b |= a[x+2][y];
    state_b |= a[x+3][y];
    state_b |= a[x+4][y];
    if state_a | state_b != 0 { return false; }

    a[x][y] = idx;
    a[x+1][y] = idx;
    a[x+1][y+1] = idx;
    a[x+2][y] = idx;
    a[x+3][y] = idx;
    a[x+4][y] = idx;

    true
}

pub fn u1(x: usize, y: usize, a: &mut [Vec<u16>]) {
    a[x][y] = 0;
    a[x+1][y] = 0;
    a[x+1][y+1] = 0;
    a[x+2][y] = 0;
    a[x+3][y] = 0;
    a[x+4][y] = 0;
}

#[must_use]
pub fn t2(x: usize, y: usize, a: &mut [Vec<u16>], idx: u16) -> bool {
    if x == 0 || x + 3 >= X_MAX || y + 1 >= Y_MAX { return false; }


    let mut state_a = 0;
    let mut state_b = 0;
    state_a |= a[x][y];
    state_a |= a[x-1][y+1];
    state_a |= a[x][y+1];
    state_b |= a[x+1][y+1];
    state_b |= a[x+2][y+1];
    state_b |= a[x+3][y+1];
    if state_a | state_b != 0 { return false; }

    a[x][y] = idx;
    a[x-1][y+1] = idx;
    a[x][y+1] = idx;
    a[x+1][y+1] = idx;
    a[x+2][y+1] = idx;
    a[x+3][y+1] = idx;

    true
}

pub fn u2(x: usize, y: usize, a: &mut [Vec<u16>]) {
    a[x][y] = 0;
    a[x-1][y+1] = 0;
    a[x][y+1] = 0;
    a[x+1][y+1] = 0;
    a[x+2][y+1] = 0;
    a[x+3][y+1] = 0;
}

#[must_use]
pub fn t3(x: usize, y: usize, a: &mut [Vec<u16>], idx: u16) -> bool {
    if x + 4 >= X_MAX || y + 1 >= Y_MAX { return false; }

    let mut state_a = 0;
    let mut state_b = 0;
    state_a |= a[x][y];
    state_a |= a[x+1][y];
    state_a |= a[x+2][y];
    state_b |= a[x+3][y];
    state_b |= a[x+3][y+1];
    state_b |= a[x+4][y];
    if state_a | state_b != 0 { return false; }

    a[x][y] = idx;
    a[x+1][y] = idx;
    a[x+2][y] = idx;
    a[x+3][y] = idx;
    a[x+3][y+1] = idx;
    a[x+4][y] = idx;

    true
}

pub fn u3(x: usize, y: usize, a: &mut [Vec<u16>]) {
    a[x][y] = 0;
    a[x+1][y] = 0;
    a[x+2][y] = 0;
    a[x+3][y] = 0;
    a[x+3][y+1] = 0;
    a[x+4][y] = 0;
}

#[must_use]
pub fn t4(x: usize, y: usize, a: &mut [Vec<u16>], idx: u16) -> bool {
    if x <= 2 || x + 1 >= X_MAX || y + 1 >= Y_MAX { return false; }

    let mut state_a = 0;
    let mut state_b = 0;
    state_a |= a[x][y];
    state_a |= a[x][y+1];
    state_a |= a[x+1][y+1];
    state_b |= a[x-1][y+1];
    state_b |= a[x-2][y+1];
    state_b |= a[x-3][y+1];
    if state_a | state_b != 0 { return false; }

    a[x][y] = idx;
    a[x][y+1] = idx;
    a[x+1][y+1] = idx;
    a[x-1][y+1] = idx;
    a[x-2][y+1] = idx;
    a[x-3][y+1] = idx;

    true
}

pub fn u4(x: usize, y: usize, a: &mut [Vec<u16>]) {
    a[x][y] = 0;
    a[x][y+1] = 0;
    a[x+1][y+1] = 0;
    a[x-1][y+1] = 0;
    a[x-2][y+1] = 0;
    a[x-3][y+1] = 0;
}

#[must_use]
pub fn t5(x: usize, y: usize, a: &mut [Vec<u16>], idx: u16) -> bool {
    if x == 0 || y + 4 >= Y_MAX { return false; }

    let mut state_a = 0;
    let mut state_b = 0;
    state_a |= a[x][y];
    state_a |= a[x][y+1];
    state_a |= a[x-1][y+1];
    state_b |= a[x][y+2];
    state_b |= a[x][y+3];
    state_b |= a[x][y+4];
    if state_a | state_b != 0 { return false; }


    a[x][y] = idx;
    a[x][y+1] = idx;
    a[x-1][y+1] = idx;
    a[x][y+2] = idx;
    a[x][y+3] = idx;
    a[x][y+4] = idx;

    true
}

pub fn u5(x: usize, y: usize, a: &mut [Vec<u16>]) {
    a[x][y] = 0;
    a[x][y+1] = 0;
    a[x-1][y+1] = 0;
    a[x][y+2] = 0;
    a[x][y+3] = 0;
    a[x][y+4] = 0;
}

#[must_use]
pub fn t6(x: usize, y: usize, a: &mut [Vec<u16>], idx: u16) -> bool {
    if x + 1 >= X_MAX || y + 4 >= Y_MAX { return false; }

    let mut state_a = 0;
    let mut state_b = 0;
    state_a |= a[x][y];
    state_a |= a[x][y+1];
    state_a |= a[x+1][y+1];
    state_b |= a[x][y+2];
    state_b |= a[x][y+3];
    state_b |= a[x][y+4];
    if state_a | state_b != 0 { return false; }


    a[x][y] = idx;
    a[x][y+1] = idx;
    a[x+1][y+1] = idx;
    a[x][y+2] = idx;
    a[x][y+3] = idx;
    a[x][y+4] = idx;

    true
}

pub fn u6(x: usize, y: usize, a: &mut [Vec<u16>]) {
    a[x][y] = 0;
    a[x][y+1] = 0;
    a[x+1][y+1] = 0;
    a[x][y+2] = 0;
    a[x][y+3] = 0;
    a[x][y+4] = 0;
}

#[must_use]
pub fn t7(x: usize, y: usize, a: &mut [Vec<u16>], idx: u16) -> bool {
    if x == 0 || y + 4 >= Y_MAX { return false; }

    let mut state_a = 0;
    let mut state_b = 0;
    state_a |= a[x][y];
    state_a |= a[x][y+1];
    state_a |= a[x][y+2];
    state_b |= a[x][y+3];
    state_b |= a[x-1][y+3];
    state_b |= a[x][y+4];
    if state_a | state_b != 0 { return false; }

    a[x][y] = idx;
    a[x][y+1] = idx;
    a[x][y+2] = idx;
    a[x][y+3] = idx;
    a[x-1][y+3] = idx;
    a[x][y+4] = idx;

    true
}

pub fn u7(x: usize, y: usize, a: &mut [Vec<u16>]) {
    a[x][y] = 0;
    a[x][y+1] = 0;
    a[x][y+2] = 0;
    a[x][y+3] = 0;
    a[x-1][y+3] = 0;
    a[x][y+4] = 0;
}

#[must_use]
pub fn t8(x: usize, y: usize, a: &mut [Vec<u16>], idx: u16) -> bool {
    if x + 1 >= X_MAX || y + 4 >= Y_MAX { return false; }

    let mut state_a = 0;
    let mut state_b = 0;
    state_a |= a[x][y];
    state_a |= a[x][y+1];
    state_a |= a[x][y+2];
    state_b |= a[x][y+3];
    state_b |= a[x+1][y+3];
    state_b |= a[x][y+4];
    if state_a | state_b != 0 { return false; }

    a[x][y] = idx;
    a[x][y+1] = idx;
    a[x][y+2] = idx;
    a[x][y+3] = idx;
    a[x+1][y+3] = idx;
    a[x][y+4] = idx;

    true
}

pub fn u8(x: usize, y: usize, a: &mut [Vec<u16>]) {
    a[x][y] = 0;
    a[x][y+1] = 0;
    a[x][y+2] = 0;
    a[x][y+3] = 0;
    a[x+1][y+3] = 0;
    a[x][y+4] = 0;
}

#[allow(clippy::needless_range_loop)]
pub fn get_gap_index(x: usize, y: usize, a: &mut [Vec<u16>]) -> Option<Pos> {
    // first check till end of line
    for x_pos in x..X_MAX {
        if a[x_pos][y] == 0 { return Some(Pos { x: x_pos, y }); }
    }

    for y_pos in (y+1)..Y_MAX {
        for x_pos in 0..X_MAX {
            if a[x_pos][y_pos] == 0 { return Some(Pos { x: x_pos, y: y_pos }); }
        }
    }
    None
}
