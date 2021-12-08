use std::{process, thread};
use std::time::Duration;
use std::time::Instant;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

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
    let loading_screen = texture_creator.load_texture("assets/out_of_game/loading_screen/stone_brick_loading_sprite_sheet_192x256.png").unwrap();
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

    /*
    Setup loading screen
     */
    let mut source = Vec::new();
    for y in 0..4 {
        for x in 0..3 {
            source.push(Rect::new(x*64, y*64, 64, 64));
        }
    }
    let destination = Rect::new(
        (1280 / 2) - 32,
        (720 / 2) - 32,
        64 * 2,
        64 * 2,
    );
    let loading_duration: f32 = 1.5;
    let intervals = (1..13).map(|i| (loading_duration / 12 as f32) * i as f32);
    let loading_clock: Vec<(f32, Rect, Rect)> = intervals
        .zip(source)
        .map(|(i, s)| (i, s, destination))
        .collect();
    /*
    Loading screen setup complete
     */

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

        let time = Instant::now();

        while time.elapsed() < Duration::from_secs_f32(loading_duration){
            for frame in &loading_clock {
                let (interval, source, destination) = frame;
                if time.elapsed() < Duration::from_secs_f32(*interval) {
                    wincan.set_draw_color(Color::BLACK);
                    wincan.clear();
                    wincan.copy(&loading_screen, *source, *destination);
                    wincan.present();
                }
            }
        }
    }
}