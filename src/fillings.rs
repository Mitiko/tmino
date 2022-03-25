// OR:
// 0 0 -> 0
// 0 1 -> 1
// 1 0 -> 1
// 1 1 -> 1
#[must_use]
pub fn t1(x: usize, y: usize, a: &mut [Vec<u16>], idx: u16) -> bool {
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

#[must_use]
pub fn t2(x: usize, y: usize, a: &mut [Vec<u16>], idx: u16) -> bool {
    // TODO: Check for X range
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

#[must_use]
pub fn t3(x: usize, y: usize, a: &mut [Vec<u16>], idx: u16) -> bool {
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

#[must_use]
pub fn t4(x: usize, y: usize, a: &mut [Vec<u16>], idx: u16) -> bool {
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

#[must_use]
pub fn t5(x: usize, y: usize, a: &mut [Vec<u16>], idx: u16) -> bool {
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

#[must_use]
pub fn t6(x: usize, y: usize, a: &mut [Vec<u16>], idx: u16) -> bool {
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

#[must_use]
pub fn t7(x: usize, y: usize, a: &mut [Vec<u16>], idx: u16) -> bool {
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

#[must_use]
pub fn t8(x: usize, y: usize, a: &mut [Vec<u16>], idx: u16) -> bool {
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
