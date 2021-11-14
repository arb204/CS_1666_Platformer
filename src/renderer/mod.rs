use std::error::Error;
use sdl2::image::LoadTexture;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use crate::game::Game;

pub struct Renderer {
    wincan: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>
}

impl Renderer {
    pub fn new(wincan: WindowCanvas) -> Result<Renderer, Box<dyn Error>> {
        let texture_creator = wincan.texture_creator();
        Ok(Renderer { wincan, texture_creator })
    }

    pub fn display(&mut self, game: &Game) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn display_start_screen(&mut self) -> Result<(), String> {
        let start_screen = self.texture_creator.load_texture("assets/out_of_game/menu/start_screen.png")?;
        self.wincan.copy(&start_screen, None, None)?;
        self.wincan.present();
        Ok(())
    }
}