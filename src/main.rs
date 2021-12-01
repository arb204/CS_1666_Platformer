extern crate sdl2;

use std::env;

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
	let mut mode = networking::NetworkingMode::Send;
	if args.len() == 2 {
		if &args[1] == "mirror" {
			mode = networking::NetworkingMode::Receive;
			menu::show_menu(wincan, event_pump, mouse, mode);
		} else if &args[1] == "credits" {
			credits::show_credits(wincan, event_pump);
		}
	} else {
		menu::show_menu(wincan, event_pump, mouse, mode);
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
