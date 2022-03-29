use crate::{Pos, X_MAX, Y_MAX, TMino};

// OR:
// 0 0 -> 0
// 0 1 -> 1
// 1 0 -> 1
// 1 1 -> 1
#[must_use]
pub fn try_fill(pos: Pos, a: &mut [Vec<u16>], idx: u16, tmino: TMino) -> bool {
    let x = pos.x;
    let y = pos.y;

    // TODO: Use TT Munchers
    macro_rules! fill_table {
        (($a:literal, $b:literal) $($tail:tt)*) => {
            fill_table!(a[x+$a][y+$b], $($tail)* a[x+$a][y+$b] = idx)
        };
        ($empty_check:expr, ($a:literal, -$b:literal) $($tail:tt)*) => {
            fill_table!($empty_check | a[x+$a][y-$b], $($tail)* a[x+$a][y-$b] = idx)
        };
        ($empty_check:expr, (-$a:literal, $b:literal) $($tail:tt)*) => {
            fill_table!($empty_check | a[x-$a][y+$b], $($tail)* a[x-$a][y+$b] = idx)
        };
        ($empty_check:expr, (-$a:literal, -$b:literal) $($tail:tt)*) => {
            fill_table!($empty_check | a[x-$a][y-$b], $($tail)* a[x-$a][y-$b] = idx)
        };
        ($empty_check:expr, ($a:literal, $b:literal) $($tail:tt)*) => {
            fill_table!($empty_check | a[x+$a][y+$b], $($tail)* a[x+$a][y+$b] = idx)
        };
        ($empty_check:expr, $($fill_stmt:stmt)+) => {
            if $empty_check != 0 { return false; }
            $($fill_stmt)+
        };
    }

    match tmino {
        TMino::T1 => {
            if x + 4 >= X_MAX || y + 1 >= Y_MAX { return false; }
            fill_table!((0, 0) (1, 0) (1, 1) (2, 0) (3, 0) (4, 0));
        },
        TMino::T2 => {
            if x == 0 || x + 3 >= X_MAX || y + 1 >= Y_MAX { return false; }
            fill_table!((0, 0) (-1, 1) (0, 1) (1, 1) (2, 1) (3, 1));
        },
        TMino::T3 => {
            if x + 4 >= X_MAX || y + 1 >= Y_MAX { return false; }
            fill_table!((0, 0) (1, 0) (2, 0) (3, 0) (3, 1) (4, 0));
        },
        TMino::T4 => {
            if x <= 2 || x + 1 >= X_MAX || y + 1 >= Y_MAX { return false; }
            fill_table!((0, 0) (0, 1) (1, 1) (-1, 1) (-2, 1) (-3, 1));
        },
        TMino::T5 => {
            if x == 0 || y + 4 >= Y_MAX { return false; }
            fill_table!((0, 0) (0, 1) (-1, 1) (0, 2) (0, 3) (0, 4));
        },
        TMino::T6 => {
            if x + 1 >= X_MAX || y + 4 >= Y_MAX { return false; }
            fill_table!((0, 0) (0, 1) (1, 1) (0, 2) (0, 3) (0, 4));
        },
        TMino::T7 => {
            if x == 0 || y + 4 >= Y_MAX { return false; }
            fill_table!((0, 0) (0, 1) (0, 2) (0, 3) (-1, 3) (0, 4));
        },
        TMino::T8 => {
            if x + 1 >= X_MAX || y + 4 >= Y_MAX { return false; }
            fill_table!((0, 0) (0, 1) (0, 2) (0, 3) (1, 3) (0, 4));
        },
        _ => {
            return false;
        }
    }

    true
}

pub fn unfill(pos: Pos, a: &mut [Vec<u16>], tmino: TMino) {
    let x = pos.x;
    let y = pos.y;

    macro_rules! unfill_table {
        (($a:literal, -$b:literal) $($tail:tt)*) => {
            unfill_table!($($tail)* a[x+$a][y-$b] = 0)
        };
        ((-$a:literal, $b:literal) $($tail:tt)*) => {
            unfill_table!($($tail)* a[x-$a][y+$b] = 0)
        };
        ((-$a:literal, -$b:literal) $($tail:tt)*) => {
            unfill_table!($($tail)* a[x-$a][y-$b] = 0)
        };
        (($a:literal, $b:literal) $($tail:tt)*) => {
            unfill_table!($($tail)* a[x+$a][y+$b] = 0)
        };
        ($($unfill_stmt:stmt)+) => {
            $($unfill_stmt)+
        };
    }

    match tmino {
        TMino::T1 => { unfill_table!((0, 0) (1, 0) (1, 1) (2, 0) (3, 0) (4, 0));    },
        TMino::T2 => { unfill_table!((0, 0) (-1, 1) (0, 1) (1, 1) (2, 1) (3, 1));   },
        TMino::T3 => { unfill_table!((0, 0) (1, 0) (2, 0) (3, 0) (3, 1) (4, 0));    },
        TMino::T4 => { unfill_table!((0, 0) (0, 1) (1, 1) (-1, 1) (-2, 1) (-3, 1)); },
        TMino::T5 => { unfill_table!((0, 0) (0, 1) (-1, 1) (0, 2) (0, 3) (0, 4));   },
        TMino::T6 => { unfill_table!((0, 0) (0, 1) (1, 1) (0, 2) (0, 3) (0, 4));    },
        TMino::T7 => { unfill_table!((0, 0) (0, 1) (0, 2) (0, 3) (-1, 3) (0, 4));   },
        TMino::T8 => { unfill_table!((0, 0) (0, 1) (0, 2) (0, 3) (1, 3) (0, 4));    },
        _ => { }
    }
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
