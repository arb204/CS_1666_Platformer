
use std::collections::HashSet;


use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use std::fs;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseUtil;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

use crate::animation_controller::animation_controller::Anim;
use crate::animation_controller::animation_controller::AnimController;
use crate::animation_controller::animation_controller::Condition;
use crate::networking;
use crate::networking::NetworkingMode;
use crate::physics_controller::physics_controller::PhysicsController;
use crate::player::player::Player;
use crate::portal_controller::portal_controller::Portal;
use crate::portal_controller::portal_controller::PortalController;
use crate::rect_collider::rect_collider::RectCollider;
use crate::credits::show_credits;

const TILE_SIZE: u32 = 64;
const BACKGROUND: Color = Color::RGBA(0, 128, 128, 255);

const DOORW: u32 = 160;
const DOORH: u32 = 230;
//const DOOR_POS: (u32, u32) = (1060, 430);

// load_level: used to load a level (UNUSED FOR NOW)
pub(crate) fn parse_level(filename: &str) -> Vec<Vec<String>> {
    //this function returns a list of the different objects in our scene
    //separated into the different parameters for each object
    let mut results: Vec<Vec<String>> = vec!();
    for a in fs::read_to_string("src/levels/".to_owned()+filename).unwrap().split("\r\n").collect::<Vec<&str>>() {
        let mut result = a.split("-").collect::<Vec<&str>>();
        let mut newresult: Vec<String> = vec!();
        for r in result {
            newresult.push(r.to_string());
        }
        results.push(newresult);
    }
    results
}

pub(crate) fn show_game(mut wincan: WindowCanvas, mut event_pump: sdl2::EventPump, mouse: MouseUtil, network_mode: NetworkingMode) -> Result<(), String> {
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
    let p1collider = RectCollider::new(0.0, 0.0, 69.0, 98.0);
    let blue_portal_collider = RectCollider::new(-100.0, -100.0, 60.0, 100.0);
    let orange_portal_collider = RectCollider::new(-100.0, -100.0, 60.0, 100.0);

    let p1physcon = PhysicsController::new(75.0, 500.0, 8.0, 0.7, 20.0, 1, 0.2, 1.0, 70.0, vec!());
    let blue_portal = Portal::new(0);
    let orange_portal = Portal::new(1);
    let p1portalcon = PortalController::new(-10, 60, p1physcon.clone(), vec!(blue_portal, orange_portal), vec!(blue_portal_collider, orange_portal_collider), vec!(), vec!());

    //this is a list of the animations we'll use for the player
    //the first parameter is the frames to use
    //the second parameter is how long each frame should be drawn before progressing
    //the third is the condition to activate the animation
    //the last is a reference to its parent animation controller
    let idle = Anim::new(vec![1], vec![10, 10], Condition::new("true".to_string(), 1, p1physcon.clone()));
    let run = Anim::new(vec![1, 2], vec![10, 10], Condition::new("speed != 0".to_string(), 2, p1physcon.clone()));
    let jump = Anim::new(vec![3], vec![1], Condition::new("fallspeed < 0".to_string(), 3, p1physcon.clone()));
    let fall = Anim::new(vec![4], vec![1], Condition::new("fallspeed > 1".to_string(), 4, p1physcon.clone()));

    let p1anim = AnimController::new(3, 69, 98, vec![idle, run, jump]);

    let mut player1 = Player::new(p1sprite, p1physcon, p1collider, p1anim, p1portalcon);

    let mut flip = false;

    let mut level_cleared = false;

    let mut first_left_click = true;
    let mut first_right_click = true;

    // used to test the orientation of the portals for teleporting
    let mut portal_blue_side = -1;
    let mut portal_orange_side = -1;

    //level data
    let mut current_level = 0; // what level are we on?
    let final_level = 1; // what level is the last one?


    let mut level = parse_level("level0.txt");

    // we read in the level from a file and add the necessary colliders and stuff
    for obj in level.iter() {
        if obj[0] == "start" {
            player1.physics.set_x(obj[1].parse::<i32>().unwrap() as f32);
            player1.physics.set_y(obj[2].parse::<i32>().unwrap() as f32);
        }
        if obj[0] == "portalblock" {
            let new_collider = RectCollider::new(obj[1].parse::<i32>().unwrap() as f32, obj[2].parse::<i32>().unwrap() as f32, (obj[3].parse::<u32>().unwrap()*TILE_SIZE) as f32, (obj[4].parse::<u32>().unwrap()*TILE_SIZE) as f32);
            player1.add_collider(new_collider, true);
        }
        if obj[0] == "nonportalblock" {
            let new_collider = RectCollider::new(obj[1].parse::<i32>().unwrap() as f32, obj[2].parse::<i32>().unwrap() as f32, (obj[3].parse::<u32>().unwrap()*TILE_SIZE) as f32, (obj[4].parse::<u32>().unwrap()*TILE_SIZE) as f32);
            player1.add_collider(new_collider, false);
        }
    }

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
            //this is just until we get the level changing logic completed
            if current_level == final_level { break 'gameloop; }
            player1.reset_colliders();
            current_level += 1;
            // this is what I'm going with until I figure out how
            // to do "level"+current_level+".txt"
            if current_level == 1 {
                level = parse_level("level1.txt");
            }
            // we read in the next level
            for obj in level.iter() {
                if obj[0] == "start" {
                    player1.physics.set_x(obj[1].parse::<i32>().unwrap() as f32);
                    player1.physics.set_y(obj[2].parse::<i32>().unwrap() as f32);
                }
                if obj[0] == "portalblock" {
                    let new_collider = RectCollider::new(obj[1].parse::<i32>().unwrap() as f32, obj[2].parse::<i32>().unwrap() as f32, (obj[3].parse::<u32>().unwrap()*TILE_SIZE) as f32, (obj[4].parse::<u32>().unwrap()*TILE_SIZE) as f32);
                    player1.add_collider(new_collider, true);
                }
                if obj[0] == "nonportalblock" {
                    let new_collider = RectCollider::new(obj[1].parse::<i32>().unwrap() as f32, obj[2].parse::<i32>().unwrap() as f32, (obj[3].parse::<u32>().unwrap()*TILE_SIZE) as f32, (obj[4].parse::<u32>().unwrap()*TILE_SIZE) as f32);
                    player1.add_collider(new_collider, false);
                }
            }
            player1.unstop();
            level_cleared = false;
        }

        // draw the surfaces
        for obj in level.iter() {
            if obj[0] == "portalblock" {
                draw_surface(&mut wincan, &portal_surface, obj[1].parse().unwrap(), obj[2].parse().unwrap(), obj[3].parse().unwrap(), obj[4].parse().unwrap());
            }
            if obj[0] == "nonportalblock" {
                draw_surface(&mut wincan, &nonportal_surface, obj[1].parse().unwrap(), obj[2].parse().unwrap(), obj[3].parse().unwrap(), obj[4].parse().unwrap());
            }
        }

        draw_level_cleared_door(&mut wincan, &door_sheet, &player1, &door_collider);

        // draw_collision_boxes(&mut wincan, &player1);

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

        // now that updates are processed, we do networking and then render
        match network_mode {
            NetworkingMode::Send => {
                let socket = UdpSocket::bind("127.0.0.1:34255").expect("couldn't bind to address");
                socket.connect("127.0.0.1:34254").unwrap();
                networking::send_data(&mut player1, &socket, flip);
            }
            NetworkingMode::Receive => {
                let mut socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
                socket.connect("127.0.0.1:34255").unwrap();
                let player_pos = networking::receive_data(&mut socket);
                let p1sprite = texture_creator.load_texture("assets/sprite_sheets/characters-sprites_condensed.png").unwrap();
                render_mirrored_player(&mut wincan, p1sprite, player_pos, flip)?;
            }
        }
        render_player(&mut wincan, &mut player1, flip)?;

        for p in &player1.portal.portals {
            wincan.copy_ex(&portalsprite, Rect::new(500*p.color()+125, 0, 125, 250), Rect::new(p.x() as i32, p.y() as i32, 60, 100), p.rotation().into(), None, false, false)?;
        }

        wincan.copy_ex(if player1.portal.last_portal() == 0 { &bluewand } else { &orangewand }, None, Rect::new(player1.physics.x() as i32 + player1.portal.wand_x(), player1.physics.y() as i32 + player1.portal.wand_y(), 100, 20), player1.portal.next_rotation(event_pump.mouse_state().x(), event_pump.mouse_state().y()).into(), None, false, false)?;

        //draw a custom cursor
        wincan.copy(&cursor, None, Rect::new(event_pump.mouse_state().x()-27, event_pump.mouse_state().y()-38, 53, 75)).ok();

        //draw to the screen
        wincan.present();

        //lock the frame rate
        thread::sleep(Duration::from_millis(1000/frame_rate));
    }

    show_credits(wincan);

    // Out of game loop, return Ok
    Ok(())
}

fn render_player(wincan: &mut WindowCanvas, player1: &mut Player, flip: bool) -> Result<(), String>{
    wincan.copy_ex(&player1.sprite_sheet, player1.anim.next_anim(), Rect::new(player1.physics.x() as i32, player1.physics.y() as i32, 69, 98), 0.0, None, flip, false)
}

fn render_mirrored_player(wincan: &mut WindowCanvas, player_sprite: Texture, player_pos: (f32, f32), flip: bool) -> Result<(), String> {
    let player_rect = Rect::new(0, 2*98, 69, 98);
    wincan.copy_ex(&player_sprite, player_rect, Rect::new(player_pos.0 as i32, player_pos.1 as i32, 69, 98), 0.0, None, flip, false)
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

// displays a green border around player and portals collider boxes for easier debugging
fn draw_collision_boxes(wincan: &mut WindowCanvas, player: &Player) {
    wincan.set_draw_color(Color::RGBA(0, 255, 0, 255));
    wincan.draw_rect(Rect::new(player.collider.x() as i32, player.collider.y() as i32, player.collider.width() as u32, player.collider.height() as u32)).ok();
    wincan.draw_rect(Rect::new(player.portal.portal_colliders[0].x() as i32, player.portal.portal_colliders[0].y() as i32, player.portal.portal_colliders[0].width() as u32, player.portal.portal_colliders[0].height() as u32)).ok();
    wincan.draw_rect(Rect::new(player.portal.portal_colliders[1].x() as i32, player.portal.portal_colliders[1].y() as i32, player.portal.portal_colliders[1].width() as u32, player.portal.portal_colliders[1].height() as u32)).ok();
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
