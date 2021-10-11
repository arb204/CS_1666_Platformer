extern crate sdl2;

mod credits;
mod game;
mod player;
mod physics_controller;
mod rect_collider;

fn main() {
	let sdl_cxt = sdl2::init().unwrap();
	let video_subsys = sdl_cxt.video().unwrap();

	// current_scene lets the game know which section is running
	// options: mainmenu, game, credits
	let mut current_scene = "mainmenu";

	let window = video_subsys.window("credits", 1280, 720)
		.build()
		.map_err(|e| e.to_string())
		.unwrap();

	let wincan = window.into_canvas().accelerated();

	let wincan = wincan.build()
		.map_err(|e| e.to_string())
		.unwrap();
	if current_scene == "mainmenu" {
		//main menu code goes here
	}
	if current_scene == "game" {
		//game code goes here
		game::show_game(wincan);
	}else if current_scene == "credits" {
		credits::show_credits(wincan);
	}
}
