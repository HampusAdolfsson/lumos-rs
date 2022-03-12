pub struct RenderBuffer {
    pub data: Vec<u8>,
}

pub trait RenderOutput {
    fn draw(buffer: &RenderBuffer) -> ();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
