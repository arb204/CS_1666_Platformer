extern crate sdl2;

mod credits;
mod game;
mod player;
mod physics_controller;
mod rect_collider;
mod sdl_core;
mod animation_controller;

use crate::sdl_core::SDLCore;

fn main() {
	let core = SDLCore::init("Warp Wizards", true, 1280, 720).unwrap();
	let sdl_cxt = &core.sdl_cxt;
	let video_subsys = sdl_cxt.video().unwrap();

	// current_scene lets the game know which section is running
	// options: mainmenu, game, credits
	let mut current_scene = "game";

	if current_scene == "game" {
		//main menu code goes here
	}
	if current_scene == "game" {
		//game code goes here
		game::show_game(core);
	} else if current_scene == "credits" {
		//credits::show_credits(wincan);
	}
}
