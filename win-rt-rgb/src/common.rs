
pub type RgbVec = Vec<color::RgbF32>;
pub type HsvVec = Vec<color::HsvF32>;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub left: isize,
    pub top: isize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn right(&self) -> isize {
        self.left + self.width as isize
    }
    pub fn bottom(&self) -> isize {
        self.top + self.height as isize
    }
}

