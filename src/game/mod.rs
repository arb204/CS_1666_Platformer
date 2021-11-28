use std::error::Error;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseUtil;
use crate::{Config, levels, ProgramState};
use crate::player::Player;

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
    pub player_mode: PlayerMode,
    pub player_local: Player,
    pub player_remote: Option(Player),
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
        if self.state.scene == Scene::StartScreen {
            self.init_level(1);
        } else if self.state.scene == Scene::Level(level_num) {
            if level_num != levels::LAST_LEVEL {
                self.init_level(level_num + 1);
            } else {
                self.state.scene = Scene::FinishedGame;
            }
        }

        Ok(ProgramState::Continue)
    }

    fn handle_finished_game(&mut self)
}