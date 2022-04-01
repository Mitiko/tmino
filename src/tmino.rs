#[derive(Debug, Clone, Copy)]
pub enum TMino {
    T0, T1, T2, T3, T4, T5, T6, T7, T8
}

impl TMino {
    pub fn iter(self) -> TMinoIter {
        TMinoIter(self)
    }
}

pub struct TMinoIter(TMino);
impl Iterator for TMinoIter {
    type Item = TMino;

    fn next(&mut self) -> Option<Self::Item> {
        let tnext = match self.0 {
            TMino::T0 => Some(TMino::T1),
            TMino::T1 => Some(TMino::T3),
            TMino::T3 => Some(TMino::T2),
            TMino::T2 => Some(TMino::T4),
            TMino::T4 => Some(TMino::T6),
            TMino::T6 => Some(TMino::T5),
            TMino::T5 => Some(TMino::T8),
            TMino::T8 => Some(TMino::T7),
            TMino::T7 => None
        };
        // Move state forward
        if let Some(ti) = tnext { self.0 = ti; }
        tnext
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub x: usize,
    pub y: usize
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self { Self { x, y } }
}
