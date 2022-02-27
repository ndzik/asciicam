#[derive(Copy, Clone)]
pub struct Dim {
    pub width: u32,
    pub height: u32,
}

pub trait Canvas {
    // draw draws the given buffer of chars onto the canvas.
    fn draw(&mut self, buffer: &[char]) -> Result<(), Box<dyn std::error::Error>>;

    // dim returns the canvas' dimension.
    fn dim(&self) -> Dim;

    // cell_aspect_ratio returns the aspect ratio of a single cell within the canvas. It is
    // returned as a tuple relating `width : height`.
    fn cell_aspect_ratio(&self) -> (u32, u32);
}
