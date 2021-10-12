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
use crate::sdl_core::SDLCore;

pub(crate) fn show_game(mut core: SDLCore) -> Result<(), String> {
    let texture_creator = core.wincan.texture_creator();

    let frame_rate = 60;

    let p1sprite = texture_creator.load_texture("assets/sprite_sheets/characters-sprites.png").unwrap();
    let p1physcon = PhysicsController::new("player1".to_string(), 0.0, 0.0, 6.0, 0.7, 20.0, 1, 0.2, 1.0, 7.0);
    let p1collider = RectCollider::new(0.0, 0.0, 100.0, 100.0, 0.0, true);

    let mut player1 = Player::new(p1sprite, p1physcon, p1collider);

    'gameloop: loop {
        for event in core.event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
                /*Event::KeyDown{keycode: Some(k), ..} => {
                    match k {
                        Keycode::W => player1.physics.jump(),
                        Keycode::A => player1.physics.accelerate_left(),
                        Keycode::D => player1.physics.accelerate_right(),
                        _ => {},
                    }
                } */
                _ => {},
            }
        }
        let keystate: HashSet<Keycode> = core.event_pump
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

        if player1.physics.y() > 620.0 && player1.physics.fall_speed() > 0.0 {
            player1.physics.set_grounded();
            player1.physics.reset_jumps();
            player1.physics.set_y(player1.physics.y() - player1.physics.fall_speed());
        }

        player1.physics.update();
        //player1.physics.debug();

        core.wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
        core.wincan.clear();

        core.wincan.copy(&player1.sprite_sheet, Rect::new(100, 0, 100, 100), Rect::new(player1.physics.x() as i32, player1.physics.y() as i32, 100, 100)).ok();
        core.wincan.present();

        //lock the frame rate
        thread::sleep(Duration::from_millis(1000/frame_rate));
    }

    // Out of game loop, return Ok
	Ok(())
}
