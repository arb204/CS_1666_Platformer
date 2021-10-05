extern crate sdl2;

use std::time::Duration;
use std::thread;
//use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::image::LoadTexture;

fn main() {
	let sdl_cxt = sdl2::init().unwrap();
	let video_subsys = sdl_cxt.video().unwrap();

	let window = video_subsys.window("credits", 1280, 720)
		.build()
		.map_err(|e| e.to_string())
		.unwrap();

	let wincan = window.into_canvas().accelerated();

	let mut wincan = wincan.build()
		.map_err(|e| e.to_string())
		.unwrap();

//	let event_pump = sdl_cxt.event_pump().unwrap();



	let texture_creator = wincan.texture_creator();

	let andrew = texture_creator.load_texture("images/andrew_credit_page.png").unwrap();
	let kira = texture_creator.load_texture("images/kira_credit_page.jpg").unwrap();
	let josh = texture_creator.load_texture("images/josh_credit_page.png").unwrap();
	let alvyn = texture_creator.load_texture("images/alvyn_credit_page.png").unwrap();
	let greg = texture_creator.load_texture("images/greg_credits.jpg").unwrap();
	let jake = texture_creator.load_texture("images/jake_credits.jpeg").unwrap();
	//let bryce = texture_creator.load_texture("images/bryce_credits.png").unwrap();
	//let austin = texture_creator.load_texture("images/austin_credits.png").unwrap();
	//let evan = texture_creator.load_texture("images/evan_credits.png").unwrap();

	wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
	wincan.clear();

	wincan.copy(&andrew, None, None);
	wincan.present();

	thread::sleep(Duration::from_millis(1500));

	wincan.copy(&kira, None, None);
	wincan.present();

	thread::sleep(Duration::from_millis(1500));

	wincan.copy(&josh, None, None);
	wincan.present();

	thread::sleep(Duration::from_millis(1500));

	wincan.copy(&alvyn, None, None);
	wincan.present();

	thread::sleep(Duration::from_millis(1500));

	wincan.copy(&greg, None, None);
	wincan.present();

	thread::sleep(Duration::from_millis(1500));

	wincan.copy(&jake, None, None);
	wincan.present();

	thread::sleep(Duration::from_millis(1500));
/*
	wincan.copy(&bryce, None, None);
	wincan.present();

	thread::sleep(Duration::from_millis(1500));

	wincan.copy(&austin, None, None);
	wincan.present();

	thread::sleep(Duration::from_millis(1500));

	wincan.copy(&evan, None, None);
	wincan.present();

	thread::sleep(Duration::from_millis(1500));*/
}