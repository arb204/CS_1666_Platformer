use std::collections::HashMap;
use std::error::Error;

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseUtil;

use player::Player;
use rect_collider::RectCollider;

use crate::{Config, levels, ProgramState};

pub mod physics_controller;
pub mod portal_controller;
pub mod player;
pub mod object_controller;
pub mod animation_controller;
pub mod rect_collider;

const TILE_SIZE: u32 = 64;
const DOORW: u32 = 160;
const DOORH: u32 = 230;

pub enum Scene {
    StartScreen,
    // MainMenu,
    Level (u8),
    // LoadingScreen,
    FinishedGame,
    Credits,
}

pub enum PlayerMode {
    SinglePlayer,
    MultiPlayer,
}

pub struct GameState {
    pub scene: Scene,
    player_mode: PlayerMode,
    player1: Player,
    player_remote: Option<Player>,
    door_collider: RectCollider,
    p1collider: RectCollider,
    blue_portal_collider: RectCollider,
    orange_portal_collider: RectCollider,
    block_collider: RectCollider,
    p1physcon: physics_controller::PhysicsController,
    blue_portal: portal_controller::Portal,
    orange_portal: portal_controller::Portal,
    p1portalcon: portal_controller::PortalController,
    p1anim: animation_controller::AnimController,
    block: object_controller::ObjectController,
    looking_left: bool,
    level_cleared: bool,
    first_left_click: bool,
    first_right_click: bool,
    portal_blue_side: i8,
    portal_orange_side: i8,
}

pub struct Game {
    pub state: GameState,
    event_pump: EventPump,
    mouse: MouseUtil,
}

impl Game {
    pub fn new(config: Config, event_pump: EventPump, mouse: MouseUtil) -> Game {
        // colliders
        let door_collider = RectCollider::new((1280 - DOORW + 25) as f32, (720 - DOORH + 25) as f32, (DOORW/2 - 10) as f32, (DOORH - 90) as f32);
        let p1collider = RectCollider::new(0.0, 0.0, 69.0, 98.0);
        let blue_portal_collider = RectCollider::new(-100.0, -100.0, 60.0, 100.0);
        let orange_portal_collider = RectCollider::new(-100.0, -100.0, 60.0, 100.0);
        let block_collider = RectCollider::new(200.0, (720-(3*TILE_SIZE as i32)/2) as f32, (TILE_SIZE/2) as f32, (TILE_SIZE/2) as f32);

        // controllers
        let p1physcon = physics_controller::PhysicsController::new(
            75.0,
            500.0,
            8.0,
            0.7,
            20.0,
            1,
            0.2,
            1.0,
            70.0,
            vec!()
        );
        let blue_portal = portal_controller::Portal::new(0);
        let orange_portal = portal_controller::Portal::new(1);
        let p1portalcon = portal_controller::PortalController::new(
            -10,
            60,
            p1physcon.clone(),
            vec!(blue_portal, orange_portal),
            vec!(blue_portal_collider, orange_portal_collider),
            vec!(),
            vec!()
        );


        //this is a list of the animations we'll use for the player
        //the first parameter is the frames to use
        //the second parameter is how long each frame should be drawn before progressing
        //the third is the condition to activate the animation
        //the last is a reference to its parent animation controller
        let idle = animation_controller::Anim::new(vec![1], vec![10, 10], animation_controller::Condition::new("true".to_string(), 1, p1physcon.clone()));
        let run = animation_controller::Anim::new(vec![1, 2], vec![10, 10], animation_controller::Condition::new("speed != 0".to_string(), 2, p1physcon.clone()));
        let jump = animation_controller::Anim::new(vec![3], vec![1], animation_controller::Condition::new("fallspeed < 0".to_string(), 3, p1physcon.clone()));
        let fall = animation_controller::Anim::new(vec![4], vec![1], animation_controller::Condition::new("fallspeed > 1".to_string(), 4, p1physcon.clone()));

        let p1anim = animation_controller::AnimController::new(3, 69, 98, vec![idle, run, jump, fall]);

        let mut player1 = Player::new(p1physcon, p1collider, p1anim, p1portalcon);

        let mut block = object_controller::ObjectController::new(block_collider);

        let mut looking_left = false;

        let mut level_cleared = false;

        let mut first_left_click = true;
        let mut first_right_click = true;

        // used to test the orientation of the portals for teleporting
        let mut portal_blue_side: i8 = -1;
        let mut portal_orange_side: i8= -1;

        let mut game = Game {
            state: GameState {
                scene: Scene::StartScreen,
                player_mode: PlayerMode::SinglePlayer,
                player1,
                player_remote: None,
                door_collider,
                p1collider,
                blue_portal_collider,
                orange_portal_collider,
                block_collider,
                p1physcon,
                blue_portal,
                orange_portal,
                p1portalcon,
                p1anim,
                block,
                looking_left,
                level_cleared,
                first_left_click,
                first_right_click,
                portal_blue_side,
                portal_orange_side
            },
            event_pump,
            mouse
        };

        let mut level = levels::parse_level("level1.txt");
        game.load_level_colliders_and_respawn_pos(&mut level);

        game
    }

    fn load_level_colliders_and_respawn_pos(&mut self, level: &mut Vec<Vec<String>>) {
        for obj in level.iter() {
            if obj[0] == "start" {
                self.state.player1.physics.set_start_x(obj[1].parse::<i32>().unwrap() as f32);
                self.state.player1.physics.set_start_y(obj[2].parse::<i32>().unwrap() as f32);
                self.state.player1.respawn();
            }
            if obj[0] == "portalblock" {
                let new_collider = RectCollider::new(obj[1].parse::<i32>().unwrap() as f32, obj[2].parse::<i32>().unwrap() as f32, (obj[3].parse::<u32>().unwrap() * TILE_SIZE) as f32, (obj[4].parse::<u32>().unwrap() * TILE_SIZE) as f32);
                self.state.player1.add_collider(new_collider, "portalblock");
            }
            if obj[0] == "nonportalblock" {
                let new_collider = RectCollider::new(obj[1].parse::<i32>().unwrap() as f32, obj[2].parse::<i32>().unwrap() as f32, (obj[3].parse::<u32>().unwrap() * TILE_SIZE) as f32, (obj[4].parse::<u32>().unwrap() * TILE_SIZE) as f32);
                self.state.player1.add_collider(new_collider, "nonportalblock");
            }
            if obj[0] == "portalglass" {
                let new_collider = RectCollider::new(obj[1].parse::<i32>().unwrap() as f32, obj[2].parse::<i32>().unwrap() as f32, (obj[3].parse::<u32>().unwrap() * TILE_SIZE) as f32, (obj[4].parse::<u32>().unwrap() * TILE_SIZE) as f32);
                self.state.player1.add_collider(new_collider, "portalglass");
            }
        }
    }

    pub fn update(&mut self) -> Result<ProgramState, Box<dyn Error>> {
        return match self.state {
            GameState { scene: Scene::StartScreen, .. } => { self.handle_start_scren() }
            GameState { .. } => { self.handle_single_player() }
        }
    }

    fn handle_start_scren(&mut self) -> Result<ProgramState, Box<dyn Error>> {
        // press any key to continue
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), .. } => { return Ok(ProgramState::Quit); }
                _ => {}
            }
        }
        Ok(ProgramState::Continue)
    }

    fn handle_single_player(&mut self) -> Result<ProgramState, Box<dyn Error>> {
        match self.state.scene {
            Scene::StartScreen => {self.init_level(1);}
            Scene::Level(level_num) => {
                if level_num != levels::LAST_LEVEL {
                    self.init_level(level_num + 1);
                } else {
                    self.state.scene = Scene::FinishedGame;
                }
            }
            Scene::FinishedGame => {}
            Scene::Credits => {}
        }

        Ok(ProgramState::Continue)
    }

    fn handle_finished_game(&mut self) {
        todo!();
    }
    fn init_level(&self, level_num: u8) {
        todo!()
    }
}
