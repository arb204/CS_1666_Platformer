use sdl2::render::WindowCanvas;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use std::time::Duration;

use crate::player::player::Player;
use crate::physics_controller::physics_controller::PhysicsController;
use crate::rect_collider::rect_collider::RectCollider;

pub(crate) fn show_game(mut wincan: WindowCanvas) {
    let texture_creator = wincan.texture_creator();

    let p1sprite = texture_creator.load_texture("assets/sprite_sheets/characters-sprites.png").unwrap();
    let p1physcon = PhysicsController::new("player1".to_string(), 0.0, 0.0, 5.0, 0.2, 5.0, 1, 0.3, 1.0, 7.0);
    let p1collider = RectCollider::new(0.0, 0.0, 100.0, 100.0, 0.0, true);

    let player1 = Player::new(p1sprite, p1physcon, p1collider);

    wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
    wincan.clear();
    wincan.present();

}