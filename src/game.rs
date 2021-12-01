use std::collections::HashSet;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use std::convert::TryInto;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseUtil;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

use crate::{levels, networking};
use crate::animation_controller::Anim;
use crate::animation_controller::AnimController;
use crate::animation_controller::Condition;
use crate::networking::{get_receiving_socket, get_sending_socket};
use crate::physics_controller::PhysicsController;
use crate::player::Player;
use crate::portal_controller::{Portal, PortalController};
use crate::rect_collider::RectCollider;
use crate::object_controller::ObjectController;
use crate::plate_controller::PlateController;
use crate::credits;

const TILE_SIZE: u32 = 64;
const BACKGROUND: Color = Color::RGBA(0, 128, 128, 255);

const DOORW: u32 = 160;
const DOORH: u32 = 230;
//const DOOR_POS: (u32, u32) = (1060, 430);
const FRAME_RATE: u64 = 60;

pub(crate) fn run(mut wincan: WindowCanvas, mut event_pump: sdl2::EventPump,
                  mouse: MouseUtil, network_mode: networking::NetworkingMode)
                  -> Result<(), String> {
    mouse.show_cursor(false);
    /*
    Renderer setup begins here.
    Currently only includes loading textures.
     */
    let texture_creator = wincan.texture_creator();
    // declare textures here
    let bluewand = texture_creator.load_texture("assets/in_game/player/wand/blue/wand_sprite_blue.png").unwrap();
    let orangewand = texture_creator.load_texture("assets/in_game/player/wand/orange/wand_sprite_orange.png").unwrap();
    let cursor = texture_creator.load_texture("assets/in_game/cursor/cursor.png").unwrap();
    let portalsprite = texture_creator.load_texture("assets/in_game/portal/portal-sprite-sheet.png").unwrap();
    let p1sprite = texture_creator.load_texture("assets/in_game/player/character/characters-sprites_condensed.png").unwrap();
    let stone_sprite = texture_creator.load_texture("assets/in_game/level/purple_floor/purple_floor_tile.png").unwrap();
    let door_sheet = texture_creator.load_texture("assets/in_game/level/door/doors_sprite_sheet.png").unwrap();
    let level_cleared_msg_sprite = texture_creator.load_texture("assets/in_game/message/level_cleared/level_cleared_msg.png").unwrap();
    let castle_bg = texture_creator.load_texture("assets/in_game/level/background/castle/castle-bg.png").unwrap();
    let nonportal_surface = texture_creator.load_texture("assets/in_game/level/brick/nonportal/stone_brick_64x64.png").unwrap();
    let portal_surface = texture_creator.load_texture("assets/in_game/level/brick/portal/portal_brick_64x64.png").unwrap();
    let portal_glass = texture_creator.load_texture("assets/in_game/level/brick/portal_glass.png").unwrap();
    let block_texture = texture_creator.load_texture("assets/in_game/block/block.png").unwrap();
    let pressure_plate = texture_creator.load_texture("assets/in_game/level/pressure_plate/pressure_plate_spritesheet.png").unwrap();
    let gate = texture_creator.load_texture("assets/in_game/level/gate/gate.png").unwrap();
    /*
    Renderer setup complete.
     */
    // **************************************************************************
    /*
    Game state setup begins here.
     */
    // Colliders
    let door_collider = RectCollider::new((1280 - DOORW + 25) as f32, (720 - DOORH + 25) as f32, (DOORW/2 - 10) as f32, (DOORH - 90) as f32);
    let p1collider = RectCollider::new(0.0, 0.0, 69.0, 98.0);
    let blue_portal_collider = RectCollider::new(-100.0, -100.0, 60.0, 100.0);
    let orange_portal_collider = RectCollider::new(-100.0, -100.0, 60.0, 100.0);
    let block_collider = RectCollider::new(200.0, (720-(3*TILE_SIZE as i32)/2) as f32, (TILE_SIZE/2) as f32, (TILE_SIZE/2) as f32);

    // Controllers and portals
    let p1physcon = PhysicsController::new(75.0, 500.0, 8.0, 0.7, 20.0, 1, 0.2, 1.0, 40.0, vec!());
    let blue_portal = Portal::new(0);
    let orange_portal = Portal::new(1);
    let p1portalcon = PortalController::new(-10, 60, p1physcon.clone(), vec!(blue_portal, orange_portal), vec!(blue_portal_collider, orange_portal_collider), vec!(), vec!());

    // dummy pressure plate controller, to be used later
    let mut platecon = PlateController::new(0, 0, 0, 0, 0, false);
    /*
    Animations
    the first parameter is the frames to use
    the second parameter is how long each frame should be drawn before progressing
    the third is the condition to activate the animation
    the last is a reference to its parent animation controller
    */
    let idle = Anim::new(vec![1], vec![10, 10], Condition::new("true".to_string(), 1, p1physcon.clone()));
    let run = Anim::new(vec![1, 2], vec![10, 10], Condition::new("speed != 0".to_string(), 2, p1physcon.clone()));
    let jump = Anim::new(vec![3], vec![1], Condition::new("fallspeed < 0".to_string(), 3, p1physcon.clone()));
    let fall = Anim::new(vec![4], vec![1], Condition::new("fallspeed > 1".to_string(), 4, p1physcon.clone()));

    let p1anim = AnimController::new(3, 69, 98, vec![idle, run, jump, fall]);

    // Entities
    let mut player1 = Player::new(p1physcon, p1collider, p1anim, p1portalcon);
    let mut block = ObjectController::new(block_collider);

    let mut level_cleared = false;

    // used to test the orientation of the portals for teleporting
    let mut portal_blue_side = -1;
    let mut portal_orange_side = -1;

    //level data
    let mut current_level = 0; // what level are we on?
    let final_level = 4; // what level is the last one?


    let mut level = levels::parse_level("level0.txt");
    let mut level_has_gate = false;
    // we read in the level from a file and add the necessary colliders and stuff
    for obj in level.iter() {
        if obj[0] == "start" {
            player1.physics.set_start_x(obj[1].parse::<i32>().unwrap() as f32);
            player1.physics.set_start_y(obj[2].parse::<i32>().unwrap() as f32);
            player1.respawn();
            block.set_start_pos(&player1, obj[3].parse::<i32>().unwrap() as f32, obj[4].parse::<i32>().unwrap() as f32);
            block.respawn(&player1);
        }
        if obj[0] == "portalblock" {
            let new_collider = RectCollider::new(obj[1].parse::<i32>().unwrap() as f32, obj[2].parse::<i32>().unwrap() as f32, (obj[3].parse::<u32>().unwrap()*TILE_SIZE) as f32, (obj[4].parse::<u32>().unwrap()*TILE_SIZE) as f32);
            player1.add_collider(new_collider, "portalblock");
            block.add_collider(new_collider);
        }
        if obj[0] == "nonportalblock" {
            let new_collider = RectCollider::new(obj[1].parse::<i32>().unwrap() as f32, obj[2].parse::<i32>().unwrap() as f32, (obj[3].parse::<u32>().unwrap()*TILE_SIZE) as f32, (obj[4].parse::<u32>().unwrap()*TILE_SIZE) as f32);
            player1.add_collider(new_collider, "nonportalblock");
            block.add_collider(new_collider);
        }
        if obj[0] == "portalglass" {
            let new_collider = RectCollider::new(obj[1].parse::<i32>().unwrap() as f32, obj[2].parse::<i32>().unwrap() as f32, (obj[3].parse::<u32>().unwrap()*TILE_SIZE) as f32, (obj[4].parse::<u32>().unwrap()*TILE_SIZE) as f32);
            player1.add_collider(new_collider, "portalglass");
            block.add_collider(new_collider);
        }
        if obj[0] == "gateplate" {
            platecon = PlateController::new(obj[1].parse::<i32>().unwrap(), obj[2].parse::<i32>().unwrap(), obj[3].parse::<i32>().unwrap(), obj[4].parse::<i32>().unwrap(), obj[5].parse::<i32>().unwrap(), obj[6].parse::<i32>().unwrap() == 1);
            level_has_gate = true;
        }
    }
    if !level_has_gate {
        platecon = PlateController::new(0, 0, 0, 0, 0, false);
    }
    /*
    Game state setup complete.
     */
    // ****************************************************************

    /*
    Begin game update loop.
     */
    'gameloop: loop {
        /*
        Begin game state update.
         */
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
                Event::KeyDown{keycode: Some(Keycode::S), ..} =>
                {
                    if block.carried() {
                        block.put_down(&player1);
                    }
                    else if player1.collider.is_touching(&block.collider()) {
                        block.picked_up(&player1);
                    }
                },
                Event::KeyDown{keycode: Some(Keycode::R), ..} =>
                {
                    //restart level
                    player1.respawn();
                    player1.portal.close_all();
                    block.respawn(&player1);
                },
                _ => {},
            }
        }

        let keystate: HashSet<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        move_player(&mut player1, &keystate);

        // Teleport the player
        player1.portal.teleport(&mut player1.collider, &mut player1.physics);

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
            block.reset_colliders();
            current_level += 1;
            // this is what I'm going with until I figure out how
            // to do "level"+current_level+".txt"
            let current_level_file = &format!("level{level}.txt", level = current_level);
            level = levels::parse_level(current_level_file);

            level_has_gate = false;
            // we read in the next level
            for obj in level.iter() {
                if obj[0] == "start" {
                    player1.physics.set_start_x(obj[1].parse::<i32>().unwrap() as f32);
                    player1.physics.set_start_y(obj[2].parse::<i32>().unwrap() as f32);
                    player1.respawn();
                    block.set_start_pos(&player1, obj[3].parse::<i32>().unwrap() as f32, obj[4].parse::<i32>().unwrap() as f32);
                    block.respawn(&player1);
                }
                if obj[0] == "portalblock" {
                    let new_collider = RectCollider::new(obj[1].parse::<i32>().unwrap() as f32, obj[2].parse::<i32>().unwrap() as f32, (obj[3].parse::<u32>().unwrap()*TILE_SIZE) as f32, (obj[4].parse::<u32>().unwrap()*TILE_SIZE) as f32);
                    player1.add_collider(new_collider, "portalblock");
                    block.add_collider(new_collider);
                }
                if obj[0] == "nonportalblock" {
                    let new_collider = RectCollider::new(obj[1].parse::<i32>().unwrap() as f32, obj[2].parse::<i32>().unwrap() as f32, (obj[3].parse::<u32>().unwrap()*TILE_SIZE) as f32, (obj[4].parse::<u32>().unwrap()*TILE_SIZE) as f32);
                    player1.add_collider(new_collider, "nonportalblock");
                    block.add_collider(new_collider);
                }
                if obj[0] == "portalglass" {
                    let new_collider = RectCollider::new(obj[1].parse::<i32>().unwrap() as f32, obj[2].parse::<i32>().unwrap() as f32, (obj[3].parse::<u32>().unwrap()*TILE_SIZE) as f32, (obj[4].parse::<u32>().unwrap()*TILE_SIZE) as f32);
                    player1.add_collider(new_collider, "portalglass");
                    block.add_collider(new_collider);
                }
                if obj[0] == "gateplate" {
                    platecon = PlateController::new(obj[1].parse::<i32>().unwrap(), obj[2].parse::<i32>().unwrap(), obj[3].parse::<i32>().unwrap(), obj[4].parse::<i32>().unwrap(), obj[5].parse::<i32>().unwrap(), obj[6].parse::<i32>().unwrap() == 1);
                    level_has_gate = true;
                }
            }
            if !level_has_gate {
                platecon = PlateController::new(0, 0, 0, 0, 0, false);
            }
            player1.unstop();
            level_cleared = false;
        }

        if !player1.is_dead() && (player1.physics.x() < 0.0 || player1.physics.x() > 1280.0 || player1.physics.y() < 0.0 || player1.physics.y() > 720.0) {
            player1.kill();
        }

        if player1.is_dead() {
            player1.respawn();
        }

        player1.update(platecon);
        block.update(&player1);
        platecon.update_plate(block.collider());

        // do we need to flip the player?
        player1.flip_horizontal = if level_cleared {
            player1.flip_horizontal
        } else if player1.physics.speed() > 0.0 && player1.flip_horizontal {
            false
        } else if player1.physics.speed() < 0.0 && !player1.flip_horizontal {
            true
        } else {
            player1.flip_horizontal
        };

        // create the portals
        if event_pump.mouse_state().left() {
            player1.portal.open_portal(0);
        }
        if event_pump.mouse_state().right() {
            player1.portal.open_portal(1);
        }

        // now that updates are processed, we do networking and then render
        let mut player2_data = None;

        match network_mode {
            networking::NetworkingMode::Send => {
                let socket = get_sending_socket();
                if let Err(e) = socket.connect(networking::REC_ADDR) {
                    println!("Failed to connect to {:?}", networking::REC_ADDR);
                }
                networking::pack_and_send_data(&mut player1, &socket);
            }
            networking::NetworkingMode::Receive => {
                let mut socket = get_receiving_socket();
                if let Err(e) = socket.connect(networking::SEND_ADDR) {
                    println!("Failed to connect to {:?}", networking::SEND_ADDR);
                }
                let mut buf = networking::get_packet_buffer(&mut socket);
                let player_data = networking::unpack_player_data(&mut socket, &mut buf)
                    .unwrap();

                let portal_pos: (f32, f32, f32, f32) = networking::unpack_portal_data(&mut socket, &mut buf);
                player2_data = Some((player_data, portal_pos));

                // /*for p in &player1.portal.portals {
                //     wincan.copy_ex(&posprite, Rect::new(500*p.color()+125, 0, 125, 250), Rect::new(portal_pos.0 as i32, portal_pos.1 as i32, 60, 100), 0.0, None, false, false)?;
                // }*/
                // wincan.copy_ex(&posprite, Rect::new(500*&player1.portal.portals[0].color()+125, 0, 125, 250), Rect::new(portal_pos.0 as i32, portal_pos.1 as i32, 60, 100), 0.0, None, false, false)?;
                // wincan.copy_ex(&posprite, Rect::new(500*&player1.portal.portals[1].color()+125, 0, 125, 250), Rect::new(portal_pos.2 as i32, portal_pos.3 as i32, 60, 100), 0.0, None, false, false)?;
            }
        }
        /*
        End game state update.
         */

        // **********************************************************************
        /*
        Begin rendering current frame.
         */
        wincan.set_draw_color(BACKGROUND);
        wincan.clear();
        wincan.copy(&castle_bg, None, None).ok();

        draw_level_cleared_door(&mut wincan, &door_sheet, &player1, &door_collider);
        // draw_collision_boxes(&mut wincan, &player1);
        // draw the surfaces
        for obj in level.iter() {
            if obj[0] == "portalblock" {
                draw_surface(&mut wincan, &portal_surface, obj[1].parse().unwrap(), obj[2].parse().unwrap(), obj[3].parse().unwrap(), obj[4].parse().unwrap());
            }
            if obj[0] == "nonportalblock" {
                draw_surface(&mut wincan, &nonportal_surface, obj[1].parse().unwrap(), obj[2].parse().unwrap(), obj[3].parse().unwrap(), obj[4].parse().unwrap());
            }
            if obj[0] == "portalglass" {
                draw_surface(&mut wincan, &portal_glass, obj[1].parse().unwrap(), obj[2].parse().unwrap(), obj[3].parse().unwrap(), obj[4].parse().unwrap());
            }
            if obj[0] == "gateplate" {
                draw_plate(&mut wincan, &pressure_plate, platecon);
                draw_gate(&mut wincan, &gate, platecon)
            }
        }

        draw_block(&mut wincan, &block, &block_texture);
        render_player(&p1sprite, &mut wincan, &mut player1)?;
        match player2_data {
            Some(_) => {
                let player_data = player2_data.unwrap().0;
                let player_pos: (f32, f32) = (player_data.0, player_data.1);
                let flip: bool = player_data.2;
                let anim_rect = Rect::new(
                    player_data.3,
                    player_data.4,
                    player_data.5,
                    player_data.6
                );
                render_mirrored_player(&mut wincan, &p1sprite, player_pos, flip, anim_rect)?;
            }
            None => {}
        }

        let mut render_portal = |p: &Portal| {
            wincan.copy_ex(&portalsprite, Rect::new(500*p.color()+125, 0, 125, 250), Rect::new(p.x() as i32, p.y() as i32, 60, 100), p.rotation().into(), None, false, false).unwrap();
        };
        // render portals
        for p in &player1.portal.portals {
            render_portal(p);
        }
        match player2_data {
            Some(_) => {
                let portal_data: (f32, f32, f32, f32) = player2_data.unwrap().1;
                let portal1 = (portal_data.0, portal_data.1, 0);
                let portal2 = (portal_data.2, portal_data.3, 1);
                let mut render_portal = |p: (f32, f32, i32)| {
                    wincan.copy_ex(&portalsprite, Rect::new(500*p.2+125, 0, 125, 250), Rect::new(p.0 as i32, p.1 as i32, 60, 100), 0.0, None, false, false).unwrap();
                };
                render_portal(portal1);
                render_portal(portal2);
            }
            None => {}
        }

        // wand color
        wincan.copy_ex(if player1.portal.last_portal() == 0 { &bluewand } else { &orangewand }, None, Rect::new(player1.physics.x() as i32 + player1.portal.wand_x(), player1.physics.y() as i32 + player1.portal.wand_y(), 100, 20), player1.portal.next_rotation(event_pump.mouse_state().x(), event_pump.mouse_state().y()).into(), None, false, false)?;

        //draw a custom cursor
        wincan.copy(&cursor, None, Rect::new(event_pump.mouse_state().x()-27, event_pump.mouse_state().y()-38, 53, 75)).ok();

        //draw to the screen
        wincan.present();
        /*
        End rendering current frame.
         */

        //lock the frame rate
        thread::sleep(Duration::from_millis(1000/ FRAME_RATE));
    }
    credits::show_credits(wincan, event_pump);
    Ok(())
}

fn render_player(texture: &Texture, wincan: &mut WindowCanvas, player1: &mut Player) -> Result<(), String>{
    let pos_rect = player1.physics.position_rect();
    let pos_rect = Rect::new(pos_rect.0, pos_rect.1, pos_rect.2, pos_rect.3);
    wincan.copy_ex(&texture, player1.anim.next_anim(), pos_rect, 0.0, None, player1.flip_horizontal, false)
}

fn render_mirrored_player(wincan: &mut WindowCanvas, player_sprite: &Texture, player_pos: (f32, f32), flip: bool, anim_rect: Rect) -> Result<(), String> {
    wincan.copy_ex(player_sprite, anim_rect, Rect::new(player_pos.0 as i32, player_pos.1 as i32, 69, 98), 0.0, None, flip, false)
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

fn draw_block(wincan: &mut WindowCanvas, block: &ObjectController, sprite: &Texture) {
    wincan.set_draw_color(Color::RGBA(255, 0, 0, 255));
    wincan.copy(sprite, None, Rect::new(block.x() as i32, block.y() as i32, TILE_SIZE/2, TILE_SIZE/2)).ok();
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

fn draw_plate(wincan: &mut WindowCanvas, sprite: &Texture, platecon: PlateController) {
    let x = platecon.plate_collider().x();
    let y = platecon.plate_collider().y()-TILE_SIZE as f32/2.0;
    if platecon.plate_pressed() {
        wincan.copy(sprite, Rect::new(532, 0, 266, 266), Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE)).ok();
    } else {
        wincan.copy(sprite, Rect::new(0, 0, 266, 266), Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE)).ok();
    }
}

fn draw_gate(wincan: &mut WindowCanvas, sprite: &Texture, platecon: PlateController) {
    let x = platecon.gate_x();
    let y = platecon.gate_y();
    let length = platecon.gate_length();
    if !platecon.gate_vertical() {
        if !platecon.plate_pressed() {
            wincan.copy(sprite, Rect::new(266, 0, 266, 266), Rect::new(x as i32, y as i32, length.try_into().unwrap(), TILE_SIZE)).ok();
        }
        wincan.copy(sprite, Rect::new(0, 0, 133, 266), Rect::new(x as i32, y as i32, TILE_SIZE/2, TILE_SIZE)).ok();
        wincan.copy(sprite, Rect::new(133, 0, 133, 266), Rect::new(x as i32+length-(TILE_SIZE as i32)/2, y as i32, TILE_SIZE/2, TILE_SIZE)).ok();
    } else {
        if !platecon.plate_pressed() {
            wincan.copy_ex(sprite, Rect::new(266, 0, 266, 266), Rect::new(x as i32-length/2+TILE_SIZE as i32/2, y as i32+length/2-TILE_SIZE as i32/2, length.try_into().unwrap(), TILE_SIZE), 90.0, None, false, false).ok();
        }
        wincan.copy_ex(sprite, Rect::new(0, 0, 133, 266), Rect::new(x as i32+16, y as i32-16, TILE_SIZE/2, TILE_SIZE), 90.0, None, false, false).ok();
        wincan.copy_ex(sprite, Rect::new(133, 0, 133, 266), Rect::new(x as i32+16, y as i32-16+length-(TILE_SIZE as i32)/2, TILE_SIZE/2, TILE_SIZE), 90.0, None, false, false).ok();
    }
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
