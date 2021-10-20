use sdl2::render::WindowCanvas;
use sdl2::image::LoadTexture;
use std::time::Duration;
use std::collections::HashSet;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread;

use crate::player::player::Player;
use crate::physics_controller::physics_controller::PhysicsController;
use crate::rect_collider::rect_collider::RectCollider;

const TILE_SIZE: u32 = 64;

pub(crate) fn show_game(mut wincan: WindowCanvas, mut event_pump: sdl2::EventPump) -> Result<(), String> {
    let texture_creator = wincan.texture_creator();

    let frame_rate = 60;

    let p1sprite = texture_creator.load_texture("assets/sprite_sheets/characters-sprites.png").unwrap();
    let stone_sprite = texture_creator.load_texture("assets/single_assets/stone_brick_64x64.png").unwrap();
    let p1physcon = PhysicsController::new("player1".to_string(), 0.0, 0.0, 6.0, 0.7, 20.0, 1, 0.2, 1.0, 50.0);
    let p1collider = RectCollider::new(0.0, 0.0, 100.0, 100.0, 0.0, true);

    let mut player1 = Player::new(p1sprite, p1physcon, p1collider);

    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
                _ => {},
            }
        }
        let keystate: HashSet<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        if keystate.contains(&Keycode::A) {
            player1.physics.accelerate_left();
        }
        if keystate.contains(&Keycode::D) {
            player1.physics.accelerate_right();
        }
        if keystate.contains(&Keycode::W) {
            player1.physics.jump();
        }

        if player1.physics.y() > 550.0 && player1.physics.fall_speed() > 0.0 {
            player1.physics.set_grounded();
            player1.physics.reset_jumps();
            player1.physics.set_y(player1.physics.y() - player1.physics.fall_speed());
        }

        wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
        wincan.clear();

        // Draw stone
        let mut i = 0;
        while i < 130 {
            let src = Rect::new(((i % 4) * TILE_SIZE) as i32, 0, TILE_SIZE, TILE_SIZE);
            let pos = Rect::new((i * 10) as i32, (720 - TILE_SIZE) as i32, TILE_SIZE, TILE_SIZE);

            wincan.copy(&stone_sprite, src, pos).ok();

            i += 1;
        }

        player1.physics.update();
        //player1.physics.debug();

        wincan.copy(&player1.sprite_sheet, Rect::new(100, 0, 100, 100), Rect::new(player1.physics.x() as i32, player1.physics.y() as i32, 100, 100)).ok();
        wincan.present();

        //lock the frame rate
        thread::sleep(Duration::from_millis(1000/frame_rate));
    }

    // Out of game loop, return Ok
    Ok(())
}
