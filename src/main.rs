extern crate sdl2;

use std::{env, process};
use std::error::Error;

use sdl2::{EventPump, Sdl};
use sdl2::event::Event;
use sdl2::mouse::MouseUtil;
use sdl2::render::WindowCanvas;

use crate::renderer::Renderer;

mod credits;
mod old_game;
mod menu;
mod networking;
mod hint_system;
mod portal_traversable;
mod levels;
mod renderer;
mod game;

pub struct Config {
	mode: networking::NetworkingMode
}

impl Config {
	fn new() -> Result<Config, Box<dyn Error>> {
		todo!(); // make mode check singleplayer vs multiplayer
		// although, I'd say we want that to be a main menu option
		let args: Vec<String> = env::args().collect();
		let mut mode = networking::NetworkingMode::Send;
		if args.len() == 2 && &args[1] == "mirror" {
			mode = networking::NetworkingMode::Receive;
		}

		Ok(Config { mode })
	}
}

pub enum ProgramState {
	Quit,
	Continue,
}

fn main() {
	let config = Config::new().unwrap();
	let (wincan, mut event_pump, mouse) = setup_sdl().unwrap_or_else(|err| {
		eprintln!("Error setting up sdl: {}", err);
		process::exit(1);
	});
	let mut renderer = Renderer::new(wincan).unwrap_or_else(|err| {
		eprintln!("Error creating renderer: {}", err);
		process::exit(1);
	});
	renderer.init();

	let mut game = game::Game::new(config, event_pump, mouse);

	if let Err(e) = renderer.display_start_screen() {
		eprintln!("Error displaying start screen: {}", e);
		process::exit(1);
	};

	'game_loop: loop {

		match game.update() {
			Ok(ProgramState::Quit) => { process::exit(0); }
			Ok(ProgramState::Continue) => {}
			Err(e) => {
				eprintln!("Game error: {}", e);
				process::exit(1);
			}
		}

		renderer.display(&game);
	}
}

fn setup_sdl() -> Result<(WindowCanvas, EventPump, MouseUtil), Box<dyn Error>> {
	let sdl_cxt = sdl2::init()?;
	let event_pump = sdl_cxt.event_pump()?;
	let mouse = sdl_cxt.mouse();
	let video_subsys = sdl_cxt.video()?;
	let window = video_subsys.window("Warp Wizards", 1280, 720)
		.build()
		.map_err(|e| e.to_string())?;
	let wincan = window.into_canvas().accelerated();
	let wincan = wincan.build()
		.map_err(|e| e.to_string())?;
	Ok((wincan, event_pump, mouse))
}
