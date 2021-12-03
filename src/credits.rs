use std::{process, thread};
use std::time::Duration;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;

pub(crate) fn show_credits(mut wincan: WindowCanvas, mut event_pump: sdl2::EventPump) {
    let texture_creator = wincan.texture_creator();
    let andrew = texture_creator.load_texture("assets/out_of_game/credits/andrew_credits.png").unwrap();
    let kira = texture_creator.load_texture("assets/out_of_game/credits/kira_credits.jpg").unwrap();
    let josh = texture_creator.load_texture("assets/out_of_game/credits/josh_credits.png").unwrap();
    let alvyn = texture_creator.load_texture("assets/out_of_game/credits/alvyn_credits.png").unwrap();
    let greg = texture_creator.load_texture("assets/out_of_game/credits/greg_credits.jpg").unwrap();
    let jake = texture_creator.load_texture("assets/out_of_game/credits/jake_credits.jpeg").unwrap();
    let bryce = texture_creator.load_texture("assets/out_of_game/credits/bryce_credits.png").unwrap();
    let austin = texture_creator.load_texture("assets/out_of_game/credits/austin_credits.png").unwrap();
    let evan = texture_creator.load_texture("assets/out_of_game/credits/evan_credits.png").unwrap();
    let mut credits = Vec::new();
    credits.push(andrew);
    credits.push(kira);
    credits.push(josh);
    credits.push(alvyn);
    credits.push(greg);
    credits.push(jake);
    credits.push(bryce);
    credits.push(austin);
    credits.push(evan);

    let mut check_for_quit = || {
        for _n in 1..100 {
            thread::sleep(Duration::from_millis(15));
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => {
                        process::exit(0);
                    },
                    _ => {},
                }
            }
        }
    };

    for credit in credits {
        wincan.copy(&credit, None, None).ok();
        wincan.present();
        check_for_quit();
    }
}
