use sdl2::render::WindowCanvas;

pub struct Renderer {
    wincan: WindowCanvas
}

impl Renderer {
    pub fn new(wincan: WindowCanvas) -> Result<Renderer, &str> {
        Ok(Renderer { wincan })
    }
}