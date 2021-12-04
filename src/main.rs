extern crate sdl2;

use std::{env, process};

use sdl2::{EventPump, Sdl};
use sdl2::mouse::MouseUtil;
use sdl2::render::WindowCanvas;

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
mod plate_controller;

fn main() {
	let (sdl_cxt, wincan) = setup_sdl_and_window();
	let event_pump = sdl_cxt.event_pump().unwrap();
	let mouse = sdl_cxt.mouse();

	perform_start_logic(wincan, event_pump, mouse)
}

fn perform_start_logic(wincan: WindowCanvas, event_pump: EventPump, mouse: MouseUtil) {
	let args: Vec<String> = env::args().collect();
	let mut network = None;
	if args.len() < 2 || (args.len() == 2 && &args[1] == "singleplayer") {
		menu::show_menu(wincan, event_pump, mouse, network);
	} else if args.len() == 2 && &args[1] == "credits" {
		credits::show_credits(wincan, event_pump);
	} else if args.len() == 3 && &args[1] == "multiplayer" {
		if &args[2] == "1" || &args[2] == "p1" {
			network = Some(networking::Network::new(networking::Mode::MultiplayerPlayer1));
		} else if &args[2] == "2" || &args[2] == "p2" {
			network = Some(networking::Network::new(networking::Mode::MultiplayerPlayer2));
		} else {
			println!("Must use: multiplayer 1 or multiplayer 2");
			process::exit(0);
		}
		menu::show_menu(wincan, event_pump, mouse, network);
	} else {
		println!("Invalid Arguments. Your options are:\n\
		singleplayer (by default)\nmultiplayer 1\nmultiplayer 2\nor credits.");
		process::exit(0);
	}
}

fn setup_sdl_and_window() -> (Sdl, WindowCanvas) {
	let sdl_cxt = sdl2::init().unwrap();
	let video_subsys = sdl_cxt.video().unwrap();
	let window = video_subsys.window("Warp Wizards", 1280, 720)
		.build()
		.map_err(|e| e.to_string())
		.unwrap();
	let wincan = window.into_canvas().accelerated();
	let wincan = wincan.build()
		.map_err(|e| e.to_string())
		.unwrap();
	(sdl_cxt, wincan)
}
