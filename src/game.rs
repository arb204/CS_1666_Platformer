use std::collections::HashSet;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use std::convert::TryInto;
use std::net::UdpSocket;

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
use crate::physics_controller::PhysicsController;
use crate::player::Player;
use crate::portal_controller::{Portal, PortalController};
use crate::rect_collider::RectCollider;
use crate::object_controller::ObjectController;
use crate::plate_controller::PlateController;
use crate::credits;
use crate::networking::Multiplayer;

const TILE_SIZE: u32 = 64;
// const BACKGROUND: Color = Color::RGBA(0, 128, 128, 255);

const DOORW: u32 = 160;
const DOORH: u32 = 230;
//const DOOR_POS: (u32, u32) = (1060, 430);
const FRAME_RATE: u64 = 60;
const FRAME_TIME: Duration = Duration::from_millis(1000 / FRAME_RATE);

pub(crate) fn run(mut wincan: WindowCanvas, mut event_pump: sdl2::EventPump,
                  mouse: MouseUtil, multiplayer: Option<Multiplayer>)
                  -> Result<(), String> {
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
    let door_sheet = texture_creator.load_texture("assets/in_game/level/door/doors_sprite_sheet.png").unwrap();
    let castle_bg = texture_creator.load_texture("assets/in_game/level/background/castle/castle-bg.png").unwrap();
    let nonportal_surface = texture_creator.load_texture("assets/in_game/level/brick/nonportal/stone_brick_64x64.png").unwrap();
    let portal_surface = texture_creator.load_texture("assets/in_game/level/brick/portal/portal_brick_64x64.png").unwrap();
    let portal_glass = texture_creator.load_texture("assets/in_game/level/brick/portal_glass.png").unwrap();
    let block_texture = texture_creator.load_texture("assets/in_game/block/block.png").unwrap();
    let pressure_plate = texture_creator.load_texture("assets/in_game/level/pressure_plate/pressure_plate_spritesheet.png").unwrap();
    let gate = texture_creator.load_texture("assets/in_game/level/gate/gate.png").unwrap();
    let loading_screen = texture_creator.load_texture("assets/out_of_game/loading_screen/stone_brick_loading_sprite_sheet_192x256.png").unwrap();
    let potionsprite = texture_creator.load_texture("assets/in_game/player/potions/potions.png").unwrap();
    let instructions = texture_creator.load_texture("assets/out_of_game/instructions/instructions.png").unwrap();

    /*
    Setup loading screen
     */
    let mut source = Vec::new();
    for y in 0..4 {
        for x in 0..3 {
            source.push(Rect::new(x*64, y*64, 64, 64));
        }
    }
    let destination = Rect::new(
        (1280 / 2) - 32,
        (720 / 2) - 32,
        64 * 2,
        64 * 2,
    );
    let loading_duration: f32 = 2.0;
    let intervals = (1..13).map(|i| (loading_duration / 12 as f32) * i as f32);
    let loading_clock: Vec<(f32, Rect, Rect)> = intervals
        .zip(source)
        .map(|(i, s)| (i, s, destination))
        .collect();
    /*
    Loading screen setup complete
     */
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
    let block_collider = RectCollider::new(200.0, (720-(3*TILE_SIZE as i32)/2) as f32, (TILE_SIZE/2) as f32, (TILE_SIZE/2) as f32);

    // Controllers and portals
    let p1physcon = PhysicsController::new(75.0, 500.0, 8.0, 0.7, 20.0, 2, 0.2, 1.0, 40.0, vec!());
    let blue_portal = Portal::new(0);
    let orange_portal = Portal::new(1);
    let p1portalcon = PortalController::new(-10, 60, 20, 65, p1physcon.clone(), vec!(blue_portal, orange_portal), vec!(), vec!(), vec!());

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
    let mut player = Player::new(p1physcon, p1collider, p1anim, p1portalcon);
    let mut block = ObjectController::new(block_collider);

    //level data
    let mut current_level = 0; // what level are we on?
    let final_level = 4; // what level is the last one?

    //which type of portal are we creating?
    // false = wand (raycast)
    // true = potion (curved)
    let mut throwing_portal = false;

    let mut paused = true;
    let mut last_pause_time = SystemTime::now();

    // data to help get the time elapsed and dash time
    let mut first_loop = 0;
    let mut start_time = 0;
    let mut elapsed_time = 0;
    let mut time_of_dash = 0; // the time that 'E' is pressed
    let mut currently_dashing = 0; // 0 is not dashing, 1 if dashing
    let mut first_press: i8 = 0;

    let mut level = levels::parse_level("level0.txt");
    // we read in the level from a file and add the necessary colliders and stuff
    let mut level_has_gate = false;
    for obj in level.iter() {
        let new_collider = || {
            RectCollider::new(obj[1].parse::<i32>().unwrap() as f32, obj[2].parse::<i32>().unwrap() as f32, (obj[3].parse::<u32>().unwrap() * TILE_SIZE) as f32, (obj[4].parse::<u32>().unwrap() * TILE_SIZE) as f32)
        };
        if obj[0] == "start" {
            player.physics.set_start_x(obj[1].parse::<i32>().unwrap() as f32);
            player.physics.set_start_y(obj[2].parse::<i32>().unwrap() as f32);
            player.respawn();
            block.set_start_pos(obj[3].parse::<i32>().unwrap() as f32, obj[4].parse::<i32>().unwrap() as f32);
            block.respawn();
        }
        if obj[0] == "portalblock" {
            player.add_collider(new_collider(), "portalblock");
            block.add_collider(new_collider());
        }
        if obj[0] == "nonportalblock" {
            player.add_collider(new_collider(), "nonportalblock");
            block.add_collider(new_collider());
        }
        if obj[0] == "portalglass" {
            player.add_collider(new_collider(), "portalglass");
            block.add_collider(new_collider());
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
    Networking setup
     */
    let mut remote_player = None;
    let mut send_socket: Option<UdpSocket> = None;
    let (tx, rx) = mpsc::channel();
    // let mut network_buffer: [u8; networking::PACKET_SIZE] = [0; networking::PACKET_SIZE];
    // let mut network_buffer = Arc::new(Mutex::new(network_buffer));
    if multiplayer.is_some() {
        let connection = networking::Connection::new(multiplayer.as_ref().unwrap().mode);
        send_socket = Some(connection.send_socket);
        let receive_socket = connection.receive_socket;
        thread::spawn ( move || {
            let mut buf: [u8; networking::PACKET_SIZE] = [0; networking::PACKET_SIZE];
            loop {
                match receive_socket.recv(&mut buf) {
                    Ok(amt) => {
                        if amt == networking::PACKET_SIZE {
                            tx.send(buf);
                        }
                    }
                    Err(_) => {}
                }
            }
        });
    }
    /*
    Game state setup complete.
     */
    // ****************************************************************
    let mut level_cleared_time: Option<Instant> = None;
    /*
    Begin game update loop.
     */
    'game_loop: loop {
        // Timer tick
        let tick = Instant::now();
        // get the elapsed time
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => {
                if first_loop == 0 {
                    start_time = n.as_millis();
                    first_loop += 1;
                }
                elapsed_time = n.as_millis() - start_time;
            }
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
        /*
        Process local game input
         */
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'game_loop,
                Event::KeyDown { keycode: Some(Keycode::S), .. } =>
                {
                    if block.carried {
                        block.put_down();
                    } else if player.collider.is_touching(&block.collider()) {
                        block.picked_up(&player);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } =>
                {
                    //restart level
                    player.respawn();
                    player.portal.close_all();
                    block.respawn();
                },
                Event::KeyDown { keycode: Some(Keycode::P), .. } =>
                {
                    //pause/unpause game
                    if last_pause_time+Duration::from_millis(500) < SystemTime::now() {
                        paused = !paused;
                        last_pause_time = SystemTime::now();
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::LShift), .. } => {
                    player.portal.close_all();
                }
                Event::KeyDown { keycode: Some(Keycode::LAlt), .. } => {
                        throwing_portal = !throwing_portal;
                }
                _ => {},
            }
        }
        /*
        Move to next level
         */
        if let Some(time) = level_cleared_time {
            for frame in &loading_clock {
                let (interval, source, destination) = frame;
                if time.elapsed() < Duration::from_secs_f32(*interval) {
                    wincan.set_draw_color(Color::BLACK);
                    wincan.clear();
                    wincan.copy(&loading_screen, *source, *destination);
                    wincan.present();
                    continue 'game_loop;
                }

            }
            if current_level == final_level { break 'game_loop; }
            player.reset_colliders();
            block.reset_colliders();
            current_level += 1;
            let current_level_file = &format!("level{}.txt", current_level);
            level = levels::parse_level(current_level_file);
            // we read in the level from a file and add the necessary colliders and stuff
            let mut level_has_gate = false;
            for obj in level.iter() {
                let new_collider = || {
                    RectCollider::new(obj[1].parse::<i32>().unwrap() as f32, obj[2].parse::<i32>().unwrap() as f32, (obj[3].parse::<u32>().unwrap() * TILE_SIZE) as f32, (obj[4].parse::<u32>().unwrap() * TILE_SIZE) as f32)
                };
                if obj[0] == "start" {
                    player.physics.set_start_x(obj[1].parse::<i32>().unwrap() as f32);
                    player.physics.set_start_y(obj[2].parse::<i32>().unwrap() as f32);
                    player.respawn();
                    block.set_start_pos(obj[3].parse::<i32>().unwrap() as f32, obj[4].parse::<i32>().unwrap() as f32);
                    block.respawn();
                }
                if obj[0] == "portalblock" {
                    player.add_collider(new_collider(), "portalblock");
                    block.add_collider(new_collider());
                }
                if obj[0] == "nonportalblock" {
                    player.add_collider(new_collider(), "nonportalblock");
                    block.add_collider(new_collider());
                }
                if obj[0] == "portalglass" {
                    player.add_collider(new_collider(), "portalglass");
                    block.add_collider(new_collider());
                }
                if obj[0] == "gateplate" {
                    platecon = PlateController::new(obj[1].parse::<i32>().unwrap(), obj[2].parse::<i32>().unwrap(), obj[3].parse::<i32>().unwrap(), obj[4].parse::<i32>().unwrap(), obj[5].parse::<i32>().unwrap(), obj[6].parse::<i32>().unwrap() == 1);
                    level_has_gate = true;
                }
            }
            if !level_has_gate {
                platecon = PlateController::new(0, 0, 0, 0, 0, false);
            }
            player.unstop();
            level_cleared_time = None;
        }

        let keystate: HashSet<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        move_player(&mut player, &keystate);

        // dash controller
        if keystate.contains(&Keycode::E) && currently_dashing == 0 && (elapsed_time >= time_of_dash + (6*player.physics.dash_time())) {
            time_of_dash = elapsed_time;
            currently_dashing = 1;
        }
        if currently_dashing == 1 {
            first_press += 1;
            if elapsed_time >= time_of_dash + player.physics.dash_time() {
                currently_dashing = 0;
                first_press = 0;
                player.physics.stop_dash();
            }
            else {
                player.physics.dash(player.physics.speed(), first_press);
            }
        }

        // Teleport the player
        player.portal.teleport(&mut player.collider, &mut player.physics);

        /*
       Local Game Input Processed
        */
        // *************************************************************************
        /*
        Begin Networking
         */
        if multiplayer.is_some() {
            // send
            if send_socket.is_some() {
                let buf = networking::pack_data(&mut player, &block, &multiplayer);
                if let Err(e) = send_socket.as_ref().unwrap().send(&buf) {
                    eprintln!("Failed sending game data to other player: {}", e);
                };
            }

            // try receive
            match rx.try_recv() {
                Ok(mut buf) =>{
                    let player_data = networking::unpack_player_data(&mut buf).unwrap();
                    let portal_data: (f32, f32, f32) = networking::unpack_portal_data(&mut buf);
                    let block_data: (i32, i32, bool) = networking::unpack_block_data(&mut buf);
                    let wand_data: (i32, i32, f32) = networking::unpack_wand_data(&mut buf);
                    remote_player = Some((player_data, portal_data, block_data, wand_data));
                }
                Err(e) => {
                    match e {
                        TryRecvError::Empty => {}
                        TryRecvError::Disconnected => {
                            eprintln!("{}", e);
                            break 'game_loop;
                        }
                    }
                }
            }
        }
        /*
        End Networking
         */
        // *************************************************************************************
        /*
        Begin Game State Update:
        Check if level is cleared, kill condition, respawn condition, flip horizontal,
        and update portals.
         */
        let mut remote_player_collider = RectCollider::new(-300.0, -300.0, 69.0, 98.0);

        // kill condition
        if !player.is_dead() && (player.physics.x() < 0.0 || player.physics.x() > 1280.0 || player.physics.y() < 0.0 || player.physics.y() > 720.0) {
            player.kill();
        }

        // respawn condition
        if player.is_dead() {
            player.respawn();
        }

        player.update(platecon);
        block.update(&player);
        platecon.update_plate(block.collider());

        // do we need to flip the player?
        player.flip_horizontal =
            if player.physics.speed() > 0.0 && player.flip_horizontal {
                false
            } else if player.physics.speed() < 0.0 && !player.flip_horizontal {
                true
            } else {
                player.flip_horizontal
            };

        // create the portals
        if remote_player.is_some() {
            let network = multiplayer.as_ref().unwrap();
            match network.mode {
                networking::Mode::MultiplayerPlayer1 => {
                    if event_pump.mouse_state().left() {
                        player.portal.open_portal(0);
                    }
                    let remote_portal = remote_player.unwrap().1;
                    if remote_portal.0 != 0.0 && remote_portal.1 != 0.0 {
                        player.portal.portals[1].open(remote_portal.0, remote_portal.1, remote_portal.2);
                    }
                },
                networking::Mode::MultiplayerPlayer2 => {
                    if event_pump.mouse_state().left() {
                        player.portal.open_portal(1);
                    }
                    let remote_portal = remote_player.unwrap().1;
                    if remote_portal.0 != 0.0 && remote_portal.1 != 0.0 {
                        player.portal.portals[0].open(remote_portal.0, remote_portal.1, remote_portal.2);
                    }
                }
            }
        } else {
            if event_pump.mouse_state().left() {
                if throwing_portal {
                    player.portal.throw_potion(0, event_pump.mouse_state().x(), event_pump.mouse_state().y());
                } else {
                    player.portal.open_portal(0);
                }
            }
            if event_pump.mouse_state().right() {
                if throwing_portal {
                    player.portal.throw_potion(1, event_pump.mouse_state().x(), event_pump.mouse_state().y());
                } else {
                    player.portal.open_portal(1);
                }
            }
        }
        if remote_player.is_some() {
            let remote_player = remote_player.unwrap().0;
            remote_player_collider = RectCollider::new(remote_player.0, remote_player.1, 69.0, 98.0);
            if level_cleared_time.is_none() && player.collider.is_touching(&door_collider) && remote_player_collider.is_touching(&door_collider) {
                level_cleared_time = Some(Instant::now());
                player.stop();
            }
        } else {
            // check to see if player has reached the end of the level
            if level_cleared_time.is_none() && player.collider.is_touching(&door_collider) {
                level_cleared_time = Some(Instant::now());
                player.stop();
            }
        }
        /*
        End game state update.
         */

        // **********************************************************************
        /*
        Begin rendering current frame.
         */

        wincan.copy(&castle_bg, None, None).ok();

        draw_level_cleared_door(&mut wincan, &door_sheet, &player, &door_collider, &multiplayer, &remote_player_collider);
        // draw_collision_boxes(&mut wincan, &player1);
        // draw the surfaces
        for obj in level.iter() {
            let mut draw_surface = |surface_texture: &Texture| {
                draw_surface(
                    &mut wincan,
                    surface_texture,
                    obj[1].parse().unwrap(),
                    obj[2].parse().unwrap(),
                    obj[3].parse().unwrap(),
                    obj[4].parse().unwrap()
                );
            };
            if obj[0] == "portalblock" {
                draw_surface(&portal_surface);
            }
            if obj[0] == "nonportalblock" {
                draw_surface(&nonportal_surface);
            }
            if obj[0] == "portalglass" {
                draw_surface(&portal_glass);
            }
            if obj[0] == "gateplate" {
                draw_plate(&mut wincan, &pressure_plate, platecon);
                draw_gate(&mut wincan, &gate, platecon)
            }
        }

        match remote_player {
            None => {
                draw_block(&mut wincan, &block, &block_texture);
            },
            Some(_) => {
                let block_data = remote_player.unwrap().2;
                wincan.set_draw_color(Color::RGBA(255, 0, 0, 255));
                wincan.copy(&block_texture, None, Rect::new(block_data.0, block_data.1, TILE_SIZE / 2, TILE_SIZE / 2)).ok();
            }
        }

        render_player(&p1sprite, &mut wincan, &mut player, &multiplayer)?;
        match remote_player {
            Some(_) => {
                let player_data = remote_player.unwrap().0;
                let player_pos: (f32, f32) = (player_data.0, player_data.1);
                let flip: bool = player_data.2;
                let anim_rect = Rect::new(
                    player_data.3,
                    player_data.4,
                    player_data.5,
                    player_data.6
                );
                render_remote_player(&mut wincan, &p1sprite, player_pos, flip, anim_rect)?;
            }
            None => {}
        }

        let mut render_portal = |p: &Portal| {
            wincan.copy_ex(&portalsprite, Rect::new(500 * p.color() + 125, 0, 125, 250), Rect::new(p.x() as i32, p.y() as i32, 60, 100), p.rotation().into(), None, false, false).unwrap();
        };
        // render portals
        for p in &player.portal.portals {
            render_portal(p);
        }

        //Wand Rendering
        match remote_player {
            Some(_) => {
                let wand_data = remote_player.unwrap().3;
                let player_data = remote_player.unwrap().0;
                let network = multiplayer.as_ref().unwrap();
                match network.mode {
                    networking::Mode::MultiplayerPlayer1 => {
                        wincan.copy_ex(&bluewand , None, Rect::new(player.physics.x() as i32 + player.portal.wand_x(), player.physics.y() as i32 + player.portal.wand_y(), 100, 20), player.portal.next_rotation(event_pump.mouse_state().x(), event_pump.mouse_state().y()).into(), None, false, false)?;
                        wincan.copy_ex(&orangewand, None, Rect::new(player_data.0 as i32 + wand_data.0, player_data.1 as i32 + wand_data.1, 100, 20), wand_data.2 as f64, None, false, false)?;
                    }
                    networking::Mode::MultiplayerPlayer2 => {
                        wincan.copy_ex(&orangewand , None, Rect::new(player.physics.x() as i32 + player.portal.wand_x(), player.physics.y() as i32 + player.portal.wand_y(), 100, 20), player.portal.next_rotation(event_pump.mouse_state().x(), event_pump.mouse_state().y()).into(), None, false, false)?;
                        wincan.copy_ex(&bluewand, None, Rect::new(player_data.0 as i32 + wand_data.0, player_data.1 as i32 + wand_data.1, 100, 20), wand_data.2 as f64, None, false, false)?;
                    }
                }
            }
            None => { /*wincan.copy_ex(if player.portal.last_portal() == 0 { &bluewand } else { &orangewand }, None, Rect::new(player.physics.x() as i32 + player.portal.wand_x(), player.physics.y() as i32 + player.portal.wand_y(), 100, 20), player.portal.next_rotation(event_pump.mouse_state().x(), event_pump.mouse_state().y()).into(), None, false, false)?;*/ }
        }

        // render potions as they fly through the air
        let mut potion_state = player.portal.get_potion_state();
        if potion_state.0.is_some() {
            let p0state = potion_state.0.unwrap();
            let p0x = p0state.0;
            let p0y = p0state.1;
            let p0r = p0state.2;
            wincan.copy_ex(&potionsprite, Rect::new(417, 0, 417, 417), Rect::new((p0x-12.5) as i32, (p0y-12.5) as i32, 25, 25), p0r, None, false, false)?;
        }
        if potion_state.1.is_some() {
            let p1state = potion_state.1.unwrap();
            let p1x = p1state.0;
            let p1y = p1state.1;
            let p1r = p1state.2;
            wincan.copy_ex(&potionsprite, Rect::new(0, 0, 417, 417), Rect::new((p1x-12.5) as i32, (p1y-12.5) as i32, 25, 25), p1r, None, false, false)?;
        }

        // wand and potions
        if throwing_portal {
            wincan.copy(&potionsprite, Rect::new((1-player.portal.last_portal()) as i32 *417, 0, 417, 417), Rect::new(player.physics.x() as i32 + player.portal.potion_x(), player.physics.y() as i32 + player.portal.potion_y(), 25, 25))?;
        } else {
            wincan.copy_ex(if player.portal.last_portal() == 0 { &bluewand } else { &orangewand }, None, Rect::new(player.physics.x() as i32 + player.portal.wand_x(), player.physics.y() as i32 + player.portal.wand_y(), 100, 20), player.portal.next_rotation(event_pump.mouse_state().x(), event_pump.mouse_state().y()).into(), None, false, false)?;
        }
        // the pause screen
        if paused {
            mouse.show_cursor(true);
            player.stop();
            wincan.copy(&instructions, None, Rect::new(240, 60, 800, 600))?;
        } else {
            player.unstop();
            mouse.show_cursor(false);
            //draw a custom cursor
            wincan.copy(&cursor, None, Rect::new(event_pump.mouse_state().x() - 27, event_pump.mouse_state().y() - 38, 53, 75)).ok();
        }

        //draw to the screen
        wincan.present();
        /*
        End rendering current frame.
         */

        let duration_to_sleep = FRAME_TIME.checked_sub(tick.elapsed());
        if duration_to_sleep.is_some() {
            // println!("elapsed time: {:?}\nduration should sleep: {:?}", tick.elapsed(), duration);
            thread::sleep(duration_to_sleep.unwrap());
        }
    } // end game_loop

    credits::show_credits(wincan, event_pump);
    Ok(())
}

fn render_player(texture: &Texture, wincan: &mut WindowCanvas, player1: &mut Player, network: &Option<Multiplayer>) -> Result<(), String>{
    let pos_rect = player1.physics.position_rect();
    let pos_rect = Rect::new(pos_rect.0, pos_rect.1, pos_rect.2, pos_rect.3);
    wincan.copy_ex(&texture, player1.anim.next_anim(network), pos_rect, 0.0, None, player1.flip_horizontal, false)
}

fn render_remote_player(wincan: &mut WindowCanvas, player_sprite: &Texture, player_pos: (f32, f32), flip: bool, anim_rect: Rect) -> Result<(), String> {
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
}

fn draw_block(wincan: &mut WindowCanvas, block: &ObjectController, sprite: &Texture) {
    wincan.set_draw_color(Color::RGBA(255, 0, 0, 255));
    wincan.copy(sprite, None, Rect::new(block.x() as i32, block.y() as i32, TILE_SIZE/2, TILE_SIZE/2)).ok();
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


fn draw_level_cleared_door(wincan: &mut WindowCanvas, door_sheet: &Texture, player: &Player, door_collider: &RectCollider, network: &Option<Multiplayer>, remote_collider: &RectCollider) {
    let pos = Rect::new((1280 - DOORW) as i32, (720 - 64 - DOORH) as i32, DOORW, DOORH);
    let src: Rect;
    if network.is_some() {
        if player.collider.is_touching(door_collider) || remote_collider.is_touching(door_collider){
            // get open door
            src = Rect::new(DOORW as i32, 0, DOORW, DOORH);
        } else {
            // get closed door
            src = Rect::new(0, 0, DOORW, DOORH);
        }
    } else {
        if player.collider.is_touching(door_collider) {
            // get open door
            src = Rect::new(DOORW as i32, 0, DOORW, DOORH);
        } else {
            // get closed door
            src = Rect::new(0, 0, DOORW, DOORH);
        }
    }
    wincan.copy(&door_sheet, src, pos).ok();
}
