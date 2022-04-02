use bmp::{Image, Pixel};
use rand::Rng;

use crate::{Pos, TMino};
const COLORS: [(u8, u8, u8); 23] = [
    (255, 255, 255),
    (0, 0, 0),
    (0, 255, 221),
    (166, 255, 0),
    (255, 0, 255),
    (255, 230, 0),
    (171, 0, 0),
    (0, 162, 255),
    (255, 0, 0),
    (0, 17, 255),
    (90, 139, 0),
    (210, 105, 30),
    (90, 139, 0),
    (0, 8, 114),
    (0, 175, 152),
    (85, 0, 141),
    (166, 255, 0),
    (255, 230, 0),
    (255, 174, 0),
    (255, 174, 0),
    (153, 0, 255),
    (255, 0, 0),
    (168, 0, 168)
];

pub fn draw(stack: &[(TMino, Pos)], name: &str) {
    println!("Found solution!");

    // TODO: Use a fixed set of colors
    let mut rng = rand::thread_rng();

    let x_max = 23;
    let y_max = 24;
    let mut img = Image::new(x_max as u32, y_max as u32);
    let mut grid = vec![vec![0; y_max]; x_max];
    apply_stack(stack, &mut grid);
    for (x, col) in grid.iter().enumerate() {
        for (y, &pix) in col.iter().enumerate() {
            let (r, g, b) = COLORS[pix as usize];
            img.set_pixel(x as u32, y as u32, px!(r, g, b));
        }
    }
    let id: u32 = rng.gen_range(0..1000);
    let _ = img.save(format!("{name}-{id}.bmp"));

    // Terminate the program
    std::process::exit(1);
}

fn apply_stack(stack: &[(TMino, Pos)], a: &mut[Vec<u16>]) {
    macro_rules! draw {
        ($pos:ident, $c:ident, (0, 0) $($tail:tt)*) => {
            draw!($pos, $c, $($tail)* a[$pos.x][$pos.y] = $c)
        };
        ($pos:ident, $c:ident, (-$a:literal, $b:literal) $($tail:tt)*) => {
            draw!($pos, $c, $($tail)* a[$pos.x-$a][$pos.y+$b] = $c)
        };
        ($pos:ident, $c:ident, ($a:literal, $b:literal) $($tail:tt)*) => {
            draw!($pos, $c, $($tail)* a[$pos.x+$a][$pos.y+$b] = $c)
        };

        ($pos:ident, $c:ident, $($fill_stmt:stmt)+) => {
            $($fill_stmt)+
        };
    }

    for (c, &(tmino, pos)) in stack.iter().enumerate() {
        let c = c % (COLORS.len() - 2) + 2;
        let c = c as u16;

        match tmino {
            TMino::T1 => { draw!(pos, c, (0, 0) (1, 0) (1, 1) (2, 0) (3, 0) (4, 0));    },
            TMino::T2 => { draw!(pos, c, (0, 0) (-1, 1) (0, 1) (1, 1) (2, 1) (3, 1));   },
            TMino::T3 => { draw!(pos, c, (0, 0) (1, 0) (2, 0) (3, 0) (3, 1) (4, 0));    },
            TMino::T4 => { draw!(pos, c, (0, 0) (0, 1) (1, 1) (-1, 1) (-2, 1) (-3, 1)); },
            TMino::T5 => { draw!(pos, c, (0, 0) (0, 1) (-1, 1) (0, 2) (0, 3) (0, 4));   },
            TMino::T6 => { draw!(pos, c, (0, 0) (0, 1) (1, 1) (0, 2) (0, 3) (0, 4));    },
            TMino::T7 => { draw!(pos, c, (0, 0) (0, 1) (0, 2) (0, 3) (-1, 3) (0, 4));   },
            TMino::T8 => { draw!(pos, c, (0, 0) (0, 1) (0, 2) (0, 3) (1, 3) (0, 4));    },
            _ => { }
        }
    }
}

pub fn draw_optim(stack: &[(TMino, Pos)], cells: &[Vec<bool>], name: &str) {
    // TODO: Use a fixed set of colors
    let mut rng = rand::thread_rng();

    let x_max = cells.len();
    let y_max = cells[0].len();
    let mut img = Image::new(x_max as u32, y_max as u32);
    let mut grid = vec![vec![0; y_max]; x_max];
    apply_stack(stack, &mut grid);
    for (x, col) in grid.iter().enumerate() {
        for (y, &pix) in col.iter().enumerate() {
            let color = if pix == 0 { cells[x][y] as usize }
            else { pix as usize };

            let (r, g, b) = COLORS[color];
            img.set_pixel(x as u32, y as u32, px!(r, g, b));
        }
    }

    let id: u32 = rng.gen_range(0..1000);
    let _ = img.save(format!("{name}-{id}.bmp"));
}
