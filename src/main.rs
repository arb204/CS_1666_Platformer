extern crate sdl2;

mod credits;

fn main() {
	let sdl_cxt = sdl2::init().unwrap();
	let video_subsys = sdl_cxt.video().unwrap();

	let window = video_subsys.window("credits", 1280, 720)
		.build()
		.map_err(|e| e.to_string())
		.unwrap();

	let wincan = window.into_canvas().accelerated();

	let wincan = wincan.build()
		.map_err(|e| e.to_string())
		.unwrap();

	credits::show_credits(wincan);
}
