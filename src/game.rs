use sdl2::render::{Texture, WindowCanvas};
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

<<<<<<< HEAD
const TILE_SIZE: u32 = 125;
=======
const TILE_SIZE: u32 = 64;
const BACKGROUND: Color = Color::RGBA(0, 128, 128, 255);

const DOORW: u32 = 160;
const DOORH: u32 = 230;
const DOOR_POS: (u32, u32) = (1060, 430);
>>>>>>> upstream/main

pub(crate) fn show_game(mut wincan: WindowCanvas, mut event_pump: sdl2::EventPump) -> Result<(), String> {
    let texture_creator = wincan.texture_creator();

    let frame_rate = 60;

    let p1sprite = texture_creator.load_texture("assets/sprite_sheets/characters-sprites.png").unwrap();
<<<<<<< HEAD
    let stone_sheet = texture_creator.load_texture("assets/sprite_sheets/floor-sprite.png").unwrap();
=======
    let stone_sprite = texture_creator.load_texture("assets/single_assets/stone_brick_64x64.png").unwrap();
    let door_sheet = texture_creator.load_texture("assets/sprite_sheets/doors_sprite_sheet.png").unwrap();
    let level_cleared_msg_sprite = texture_creator.load_texture("assets/single_assets/level_cleared_msg.png").unwrap();
>>>>>>> upstream/main
    let p1physcon = PhysicsController::new("player1".to_string(), 0.0, 0.0, 6.0, 0.7, 20.0, 1, 0.2, 1.0, 50.0);
    let p1collider = RectCollider::new(0.0, 0.0, 100.0, 100.0, 0.0, true);

    let mut player1 = Player::new(p1sprite, p1physcon, p1collider);
    let mut level_cleared = false;

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

        move_player(&mut player1, &keystate);
        stop_player_at_ground(&mut player1);

        wincan.set_draw_color(BACKGROUND);
        wincan.clear();

        draw_stone_floor(&mut wincan, &stone_sprite);

        draw_level_cleared_door(&mut wincan, &door_sheet, &player1);
        if level_cleared == false && player_hit_door(&player1) {
            level_cleared = true;
        }
        if level_cleared {
            draw_level_cleared_msg(&mut wincan, &level_cleared_msg_sprite);
        }

<<<<<<< HEAD
        wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
        wincan.clear();
=======
        player1.physics.update();
        // player1.physics.debug();
>>>>>>> upstream/main

        // Draw stone
        let mut i = 0;
        while i < 130 {
            let src = Rect::new(((i % 4) * TILE_SIZE) as i32, 0, TILE_SIZE, TILE_SIZE);
            let pos = Rect::new((i * 10) as i32, (720 - TILE_SIZE) as i32, TILE_SIZE, TILE_SIZE);

            wincan.copy(&stone_sheet, src, pos).ok();

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

fn move_player(player: &mut Player, keystate: &HashSet<Keycode>) {
    if keystate.contains(&Keycode::A) {
        player.physics.accelerate_left();
    }
    if keystate.contains(&Keycode::D) {
        player.physics.accelerate_right();
    }
    if keystate.contains(&Keycode::W) {
        player.physics.jump();
    }
}

fn stop_player_at_ground(player: &mut Player) {
    if player.physics.y() > 550.0 && player.physics.fall_speed() > 0.0 {
        player.physics.set_grounded();
        player.physics.reset_jumps();
        player.physics.set_y(player.physics.y() - player.physics.fall_speed());
    }
}

fn draw_stone_floor(wincan: &mut WindowCanvas, stone_sprite: &Texture) {
    let mut i = 0;
    while i < 130 {
        let src = Rect::new(((i % 4) * TILE_SIZE) as i32, 0, TILE_SIZE, TILE_SIZE);
        let pos = Rect::new((i * 10) as i32, (720 - TILE_SIZE) as i32, TILE_SIZE, TILE_SIZE);
        wincan.copy(&stone_sprite, src, pos).ok();
        i += 1;
    }
}

fn player_hit_door(player: &Player) -> bool {
    player.physics.x() > DOOR_POS.0 as f32 && player.physics.y() > DOOR_POS.1 as f32
}

fn draw_level_cleared_door(wincan: &mut WindowCanvas, door_sheet: &Texture, player: &Player) {
    let pos = Rect::new((1280 - DOORW) as i32, (720 - 64 - DOORH) as i32, DOORW, DOORH);
    let mut src: Rect;
    if player_hit_door(player){
        // get open door
        src = Rect::new(DOORW as i32, 0, DOORW, DOORH);
    } else {
        // get closed door
        src = Rect::new(0, 0, DOORW, DOORH);
    }
    wincan.copy(&door_sheet, src, pos).ok();
}

fn draw_level_cleared_msg(wincan: &mut WindowCanvas, level_cleared_msg_sprite: &Texture) {
    let src = Rect::new(0, 0, 1280, 720);
    let pos =  Rect::new(0, 0, 1280, 720);
    wincan.copy(&level_cleared_msg_sprite, src, pos);
}
