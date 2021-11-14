extern crate sdl2;

use std::{env, process};
use std::error::Error;

use sdl2::{EventPump, Sdl};
use sdl2::event::Event;
use sdl2::mouse::MouseUtil;
use sdl2::render::WindowCanvas;
use crate::renderer::Renderer;

mod credits;
mod game;
mod player;
mod physics_controller;
mod rect_collider;
mod menu;
mod animation_controller;
mod portal_controller;
mod networking;
mod hint_system;
mod object_controller;
mod portal_traversible;
mod levels;
mod renderer;

struct Config {
	mode: networking::NetworkingMode
}

impl Config {
	fn new() -> Result<Config, Box<dyn Error>> {
		let args: Vec<String> = env::args().collect();
		let mut mode = networking::NetworkingMode::Send;
		if args.len() == 2 && &args[1] == "mirror" {
			mode = networking::NetworkingMode::Receive;
		}

		Ok(Config { mode })
	}
}

// Maybe someone else has a better name than MainState.
enum MainState {
	Quit,
	Pause,
	Continue,
}

fn main() {
	let config = Config::new().unwrap();
	let (wincan, mut event_pump, mouse) = setup_wincan_and_event_pump().unwrap_or_else(|err| {
		println!("Error setting up sdl: {}", err);
		process::exit(1);
	});
	let renderer = Renderer::new(wincan).unwrap_or_else(|err| {
		println!("Error creating renderer: {}", err);
		process::exit(1);
	});

	'game_loop: loop {
		if quit(&mut event_pump) {
			break 'game_loop;
		}

		game.update(&event_pump, &mouse);

		renderer.display(&game);
	}
}

fn quit(event_pump: &mut EventPump) -> bool {
	let mut should_quit = false;
	for event in event_pump.poll_iter() {
		match event {
			Event::Quit { .. } => { should_quit = true }
			_ => {}
		}
	}
	should_quit
}

fn setup_wincan_and_event_pump() -> Result<(WindowCanvas, EventPump, MouseUtil), Box<dyn Error>> {
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
