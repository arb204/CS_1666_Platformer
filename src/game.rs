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
use crate::animation_controller::animation_controller::AnimController;
use crate::animation_controller::animation_controller::Anim;
use crate::animation_controller::animation_controller::Condition;
use crate::rect_collider::rect_collider::RectCollider;

const TILE_SIZE: u32 = 64;
const BACKGROUND: Color = Color::RGBA(0, 128, 128, 255);

const DOORW: u32 = 160;
const DOORH: u32 = 230;
const DOOR_POS: (u32, u32) = (1060, 430);

pub(crate) fn show_game(mut wincan: WindowCanvas, mut event_pump: sdl2::EventPump) -> Result<(), String> {
    let texture_creator = wincan.texture_creator();

    let frame_rate = 60;

    let p1sprite = texture_creator.load_texture("assets/sprite_sheets/characters-sprites_condensed.png").unwrap();
    let p1physcon = PhysicsController::new(0.0, 0.0, 6.0, 0.7, 20.0, 1, 0.2, 1.0, 70.0);
    let p1collider = RectCollider::new(0.0, 0.0, 69.0, 98.0, true);
    let door_collider = RectCollider::new((1280 - DOORW + 25) as f32, (720 - DOORH + 25) as f32, (DOORW/2 - 10) as f32, (DOORH - 90) as f32, false);
    let floor_collider = RectCollider::new(0.0, (720 - TILE_SIZE) as f32, 1280.0, TILE_SIZE as f32, false);

    //this is a list of the animations we'll use for the player
    //the first parameter is the frames to use
    //the second parameter is how long each frame should be drawn before progressing
    //the third is the condition to activate the animation
    //the last is a reference to its parent animation controller
    let idle = Anim::new(vec![1], vec![10, 10], Condition::new("true".to_string(), 1, p1physcon));
    let run = Anim::new(vec![1, 2], vec![10, 10], Condition::new("speed != 0".to_string(), 2, p1physcon));
    let jump = Anim::new(vec![3], vec![1], Condition::new("fallspeed < 0".to_string(), 3, p1physcon));
    //let fall = Anim::new(vec![4], vec![1], Condition::new("fallspeed > 0".to_string(), 3, p1physcon));

    let p1anim = AnimController::new(4, 3, 69, 98, vec![idle, run, jump]);

    let mut player1 = Player::new(p1sprite, p1physcon, p1collider, p1anim);

    let mut flip = false;

    let stone_sprite = texture_creator.load_texture("assets/single_assets/purple_floor_tile.png").unwrap();
    let door_sheet = texture_creator.load_texture("assets/sprite_sheets/doors_sprite_sheet.png").unwrap();
    let level_cleared_msg_sprite = texture_creator.load_texture("assets/single_assets/level_cleared_msg.png").unwrap();

    let mut level_cleared = false;

    let g = Color::RGBA(0, 255, 0, 255);

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
        stop_player_at_ground(&mut player1, &floor_collider);
        // check_bounds(&mut player1);

        wincan.set_draw_color(BACKGROUND);
        wincan.clear();

        draw_stone_floor(&mut wincan, &stone_sprite);

        // check to see if player has reached the end of the level
        draw_level_cleared_door(&mut wincan, &door_sheet, &player1, &door_collider);
        if level_cleared == false && player1.collider.is_touching(&door_collider) {
            //level_cleared = true;
            //player1.physics.immobilize();
            //player1.anim.freeze();
        }
        if level_cleared {
            draw_level_cleared_msg(&mut wincan, &level_cleared_msg_sprite);
        }

        player1.physics.update();
        player1.collider.update(&player1.physics);
        player1.anim.update(player1.physics);

        wincan.copy_ex(&player1.sprite_sheet, player1.anim.next_anim(), Rect::new(player1.physics.x() as i32, player1.physics.y() as i32, 69, 98), 0.0, None, flip, false)?;

        // use the following for visually testing the rect collider
        wincan.set_draw_color(g);
        wincan.draw_rect(Rect::new(player1.physics.x() as i32, player1.physics.y() as i32, 69, 98))?;
        wincan.draw_rect(Rect::new((1280 - DOORW + 25) as i32, (720 - DOORH + 25) as i32, DOORW/2 - 10 as u32, DOORH - 90 as u32))?;
        wincan.draw_rect(Rect::new(0, (720 - TILE_SIZE) as i32, 1280, TILE_SIZE))?;

        // do we need to flip the player?
        flip = if level_cleared {
            flip
        } else if player1.physics.speed() > 0.0 && flip  {
            false
        } else if player1.physics.speed() < 0.0 && !flip {
            true
        } else {
            flip
        };
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

fn stop_player_at_ground(player: &mut Player, floor_collider: &RectCollider) {
    if player.collider.is_touching(floor_collider) && player.physics.fall_speed() > 0.0 {
        player.physics.set_grounded();
        player.physics.reset_jumps();
        player.physics.set_y(floor_collider.y() - (TILE_SIZE + 33) as f32);
        player.physics.set_fall_speed(0.0);
        // player.physics.set_y(player.physics.y() - player.physics.fall_speed());
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

fn draw_level_cleared_door(wincan: &mut WindowCanvas, door_sheet: &Texture, player: &Player, door_collider: &RectCollider) {
    let pos = Rect::new((1280 - DOORW) as i32, (720 - 64 - DOORH) as i32, DOORW, DOORH);
    let src: Rect;
    if player.collider.is_touching(door_collider) {
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
    wincan.copy(&level_cleared_msg_sprite, src, pos).ok();
}

// fn check_bounds(player: &mut Player) {
//     if player.collider.x() >= 1200.0 && player.physics.speed() > 0.0 {
//         player.physics.set_speed(0.0);
//     }
// }