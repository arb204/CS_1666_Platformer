extern crate sdl2;

use std::env;

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

fn main() {
	let sdl_cxt = sdl2::init().unwrap();
	let video_subsys = sdl_cxt.video().unwrap();
	let event_pump = sdl_cxt.event_pump().unwrap();
	let mouse = sdl_cxt.mouse();
	
	let window = video_subsys.window("Warp Wizards", 1280, 720)
		.build()
		.map_err(|e| e.to_string())
		.unwrap();

	let wincan = window.into_canvas().accelerated();

	let wincan = wincan.build()
		.map_err(|e| e.to_string())
		.unwrap();

	let args: Vec<String> = env::args().collect();
	let mut mode = networking::NetworkingMode::Send;
	if args.len() == 2 {
		if &args[1] == "mirror" {
			mode = networking::NetworkingMode::Receive;
			menu::show_menu(wincan, event_pump, mouse, mode);
		} else if &args[1] == "credits" {
			credits::show_credits(wincan);
		}
	} else {
		menu::show_menu(wincan, event_pump, mouse, mode);
	}


}
