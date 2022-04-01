use crate::{Y_MAX, Pos, TMino};


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
    for y_pos in (y+1)..Y_MAX {
        for (x_pos, col) in a.iter().enumerate() {
            if !col[y_pos] {
                return Some(Pos::new(x_pos, y_pos));
            }
        }
    }

    None
}
