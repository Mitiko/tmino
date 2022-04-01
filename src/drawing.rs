use bmp::{Image, Pixel};
use rand::Rng;

use crate::{Pos, TMino, X_MAX, Y_MAX};

pub fn draw(stack: &[(TMino, Pos)], name: &str) {
    // TODO: Use a fixed set of colors
    let mut rng = rand::thread_rng();
    let mut colors = vec![(0, 0, 0); 1_000];
    for c in colors.iter_mut() {
        let r = rng.gen_range(0..=255);
        let g = rng.gen_range(0..=255);
        let b = rng.gen_range(0..=255);
        *c = (r, g, b);
    }
    colors[0] = (255, 255, 255);

    let mut img = Image::new(X_MAX as u32, Y_MAX as u32);
    let a = apply_stack(stack);
    for x in 0..X_MAX {
        for y in 0..Y_MAX {
            let (r, g, b) = colors[a[x][y] as usize];
            img.set_pixel(x as u32, y as u32, px!(r, g, b));
        }
    }
    let id: u32 = rng.gen_range(0..1000);
    let _ = img.save(format!("{name}-{id}.bmp"));
}

fn apply_stack(stack: &[(TMino, Pos)]) -> Vec<Vec<u16>> {
    let mut a = vec![vec![0; Y_MAX]; X_MAX];

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
        let c = (c + 1) as u16;

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

    a
}
