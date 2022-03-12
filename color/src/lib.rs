#[derive(Clone, Copy, Debug)]
pub struct Rgb<T: Copy> {
    pub red: T,
    pub green: T,
    pub blue: T,
}

pub type RgbF32 = Rgb<f32>;

impl RgbF32 {
    pub fn black() -> Self {
        RgbF32{ red: 0.0, green: 0.0, blue: 0.0 }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
