use sdl2::render::WindowCanvas;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use std::time::Duration;
use std::thread;

use crate::player;
use crate::physics_controller;

pub(crate) fn show_game(mut wincan: WindowCanvas) {
    let texture_creator = wincan.texture_creator();

    let p1sprite = texture_creator.load_texture("assets/sprite_sheets/characters-sprites.png").unwrap();

    wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
    wincan.clear();

    wincan.copy(&p1sprite, None, None).ok();
    wincan.present();

}