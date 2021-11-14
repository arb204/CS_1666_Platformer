use std::error::Error;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseUtil;
use crate::{Config, ProgramState};
use crate::player::Player;

pub enum GameState {
    StartScreen,
    // MainMenu,
    SinglePlayer {
        on_level: i32
    },
    // MultiPlayer {
    //     on_level: i32
    // },
    // Paused,
}

pub struct Game {
    pub state: GameState,
    event_pump: EventPump,
    mouse: MouseUtil,
}

impl Game {
    pub fn new(config: Config, event_pump: EventPump, mouse: MouseUtil) -> Game {
        Game { state: GameState::StartScreen, event_pump, mouse }
    }

    pub fn update(&mut self) -> Result<ProgramState, Box<dyn Error>> {
        return match self.state {
            GameState::StartScreen => { self.handle_start_scren() }
            GameState::SinglePlayer { .. } => { self.handle_single_player() }
        }
    }

    fn handle_start_scren(&mut self) -> Result<ProgramState, Box<dyn Error>> {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), .. } => { return Ok(ProgramState::Quit); }
                _ => {}
            }
        }
        Ok(ProgramState::Continue)
    }

    fn handle_single_player(&self) -> Result<ProgramState, Box<dyn Error>> {
        todo!()
    }
}