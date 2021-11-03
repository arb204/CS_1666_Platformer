use std::borrow::Borrow;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::image::LoadTexture;
use std::time::Duration;
use std::collections::HashSet;
use std::hash::Hash;
use std::io::Write;
use std::net::UdpSocket;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseUtil;
use std::thread;

use crate::player::player::Player;
use crate::physics_controller::physics_controller::PhysicsController;
use crate::animation_controller::animation_controller::AnimController;
use crate::animation_controller::animation_controller::Anim;
use crate::animation_controller::animation_controller::Condition;
use crate::rect_collider::rect_collider::RectCollider;
use crate::portal_controller::portal_controller::PortalController;
use crate::portal_controller::portal_controller::Portal;

const TILE_SIZE: u32 = 64;
const BACKGROUND: Color = Color::RGBA(0, 128, 128, 255);

const DOORW: u32 = 160;
const DOORH: u32 = 230;
//const DOOR_POS: (u32, u32) = (1060, 430);

// load_level: used to load a level (UNUSED FOR NOW)
pub(crate) fn _load_level() { }

pub(crate) fn show_game(mut wincan: WindowCanvas, mut event_pump: sdl2::EventPump, mouse: MouseUtil) -> Result<(), String> {
    mouse.show_cursor(false);
    let texture_creator = wincan.texture_creator();

    let frame_rate = 60;

    // declare textures here
    let bluewand = texture_creator.load_texture("assets/single_assets/wand_sprite_blue.png").unwrap();
    let orangewand = texture_creator.load_texture("assets/single_assets/wand_sprite_orange.png").unwrap();
    let cursor = texture_creator.load_texture("assets/single_assets/cursor.png").unwrap();
    let portalsprite = texture_creator.load_texture("assets/sprite_sheets/portal-sprite-sheet.png").unwrap();
    let p1sprite = texture_creator.load_texture("assets/sprite_sheets/characters-sprites_condensed.png").unwrap();
    let stone_sprite = texture_creator.load_texture("assets/single_assets/purple_floor_tile.png").unwrap();
    let door_sheet = texture_creator.load_texture("assets/sprite_sheets/doors_sprite_sheet.png").unwrap();
    let level_cleared_msg_sprite = texture_creator.load_texture("assets/single_assets/level_cleared_msg.png").unwrap();
    let castle_bg = texture_creator.load_texture("assets/single_assets/castle-bg.png").unwrap();
    let nonportal_surface = texture_creator.load_texture("assets/single_assets/stone_brick_64x64.png").unwrap();
    let portal_surface = texture_creator.load_texture("assets/single_assets/portal_brick_64x64.png").unwrap();

    // declare colliders here
    let door_collider = RectCollider::new((1280 - DOORW + 25) as f32, (720 - DOORH + 25) as f32, (DOORW/2 - 10) as f32, (DOORH - 90) as f32);
    let floor_collider = RectCollider::new(0.0, (720 - TILE_SIZE) as f32, 1280.0, TILE_SIZE as f32);
    let ceiling_collider = RectCollider::new(0.0, 0.0, 1280.0, TILE_SIZE as f32);
    let left_wall_collider = RectCollider::new(0.0, 0.0, TILE_SIZE as f32, 720.0);
    let right_wall_collider = RectCollider::new((1280-(TILE_SIZE as i32)) as f32, 0.0, TILE_SIZE as f32, 720.0);
    let mid_platform_collider = RectCollider::new(544.0, 400.0, ((3*TILE_SIZE) as i32) as f32, ((4*TILE_SIZE) as i32) as f32);
    let p1collider = RectCollider::new(0.0, 0.0, 69.0, 98.0);
    let blue_portal_collider = RectCollider::new(-100.0, -100.0, 60.0, 100.0);
    let orange_portal_collider = RectCollider::new(-100.0, -100.0, 60.0, 100.0);

    let p1physcon = PhysicsController::new(75.0, 500.0, 6.0, 0.7, 20.0, 1, 0.2, 1.0, 70.0, vec!(floor_collider, left_wall_collider, right_wall_collider, ceiling_collider, mid_platform_collider));
    let blue_portal = Portal::new(0);
    let orange_portal = Portal::new(1);
    let p1portalcon = PortalController::new(-10, 60, p1physcon.clone(), vec!(blue_portal, orange_portal), vec!(blue_portal_collider, orange_portal_collider), vec!(floor_collider, left_wall_collider, right_wall_collider, ceiling_collider), vec!(mid_platform_collider));

    //this is a list of the animations we'll use for the player
    //the first parameter is the frames to use
    //the second parameter is how long each frame should be drawn before progressing
    //the third is the condition to activate the animation
    //the last is a reference to its parent animation controller
    let idle = Anim::new(vec![1], vec![10, 10], Condition::new("true".to_string(), 1, p1physcon.clone()));
    let run = Anim::new(vec![1, 2], vec![10, 10], Condition::new("speed != 0".to_string(), 2, p1physcon.clone()));
    let jump = Anim::new(vec![3], vec![1], Condition::new("fallspeed < 0".to_string(), 3, p1physcon.clone()));
    //let fall = Anim::new(vec![4], vec![1], Condition::new("fallspeed > 0".to_string(), 3, p1physcon));

    let p1anim = AnimController::new(3, 69, 98, vec![idle, run, jump]);

    let mut player1 = Player::new(p1sprite, p1physcon, p1collider, p1anim, p1portalcon);

    let mut flip = false;

    let mut level_cleared = false;

    let mut first_left_click = true;
    let mut first_right_click = true;

    // used to test the orientation of the portals for teleporting
    let mut portal_blue_side = -1;
    let mut portal_orange_side = -1;

    let socket = UdpSocket::bind("127.0.0.1:34255").expect("couldn't bind to address");

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

        send_to_mirror(&player1, &socket, "127.0.0.1:34254");

        move_player(&mut player1, &keystate);

        // Is the player touching a portal?
        player1.portal.teleport(&mut player1.collider, &mut player1.physics, &portal_blue_side, &portal_orange_side);

        wincan.set_draw_color(BACKGROUND);
        wincan.clear();
        wincan.copy(&castle_bg, None, None).ok();

        draw_stone_floor(&mut wincan, &stone_sprite);

        // check to see if player has reached the end of the level
        if level_cleared == false && player1.collider.is_touching(&door_collider) {
            level_cleared = true;
            player1.stop();
        }
        if level_cleared {
            draw_level_cleared_msg(&mut wincan, &level_cleared_msg_sprite);
            //level_cleared = false;
        }

        // draw the surfaces
        draw_surface(&mut wincan, &portal_surface, 0, 0, 1, 11);
        draw_surface(&mut wincan, &portal_surface, 0, 0, 20, 1);
        draw_surface(&mut wincan, &portal_surface, 1280-TILE_SIZE as i32, 0, 1, 11);
        draw_surface(&mut wincan, &nonportal_surface, 544, 400, 3, 5);

        draw_surface(&mut wincan, &portal_surface, 0, 720-TILE_SIZE as i32, 20, 1);

        draw_level_cleared_door(&mut wincan, &door_sheet, &player1, &door_collider);

        player1.update();

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

        // create the portals
        if event_pump.mouse_state().left() {
            if player1.portal.open_portal(0, &mut portal_blue_side, &mut portal_orange_side) == 1 {
                if first_left_click {
                    player1.portal.can_teleport += 1;
                    first_left_click = false;
                }
            }
        }
        if event_pump.mouse_state().right() {
            if player1.portal.open_portal(1, &mut portal_blue_side, &mut portal_orange_side) == 1 {
                if first_right_click {
                    player1.portal.can_teleport += 1;
                    first_right_click = false;
                }
            }
        }

        for p in &player1.portal.portals {
            wincan.copy_ex(&portalsprite, Rect::new(500*p.color()+125, 0, 125, 250), Rect::new(p.x() as i32, p.y() as i32, 60, 100), p.rotation().into(), None, false, false)?;
        }

        wincan.copy_ex(&player1.sprite_sheet, player1.anim.next_anim(), Rect::new(player1.physics.x() as i32, player1.physics.y() as i32, 69, 98), 0.0, None, flip, false)?;
        wincan.copy_ex(if player1.portal.last_portal() == 0 { &bluewand } else { &orangewand }, None, Rect::new(player1.physics.x() as i32 + player1.portal.wand_x(), player1.physics.y() as i32 + player1.portal.wand_y(), 100, 20), player1.portal.next_rotation(event_pump.mouse_state().x(), event_pump.mouse_state().y()).into(), None, false, false)?;

        //draw a custom cursor
        wincan.copy(&cursor, None, Rect::new(event_pump.mouse_state().x()-27, event_pump.mouse_state().y()-38, 53, 75)).ok();

        //draw to the screen
        wincan.present();


        //lock the frame rate
        thread::sleep(Duration::from_millis(1000/frame_rate));
    }

    // Out of game loop, return Ok
    Ok(())
}

fn send_to_mirror(player: &Player, socket: &UdpSocket, address: &str) {
    socket.send_to(player.physics.x().to_ne_bytes().borrow(), address).ok();
    socket.send_to(player.physics.y().to_ne_bytes().borrow(), address).ok();
}

fn move_player(player: &mut Player, keystate: &HashSet<Keycode>) {
    if keystate.contains(&Keycode::A) {
        player.physics.accelerate_left();
    }
    if keystate.contains(&Keycode::D) {
        player.physics.accelerate_right();
    }
    if keystate.contains(&Keycode::W) || keystate.contains(&Keycode::Space) {
        player.physics.jump();
    }
    if keystate.contains(&Keycode::LShift) {
        player.portal.close_all();
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

fn draw_surface(wincan: &mut WindowCanvas, sprite: &Texture, x: i32, y: i32, width: i32, height: i32) {
    for i in 0..width {
        for j in 0..height {
            wincan.copy(sprite, None, Rect::new(x+i*TILE_SIZE as i32, y+j*TILE_SIZE as i32, TILE_SIZE, TILE_SIZE)).ok();
        }
    }
}

/*fn player_hit_door(player: &Player) -> bool {
    player.physics.x() > DOOR_POS.0 as f32 && player.physics.y() > DOOR_POS.1 as f32
}*/

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
