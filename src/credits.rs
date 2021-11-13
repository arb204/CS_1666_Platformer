use std::thread;
use std::time::Duration;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub(crate) fn show_credits(mut wincan: WindowCanvas) {
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

    wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
    wincan.clear();

    wincan.copy(&andrew, None, None).ok();
    wincan.present();

    thread::sleep(Duration::from_millis(1500));

    wincan.copy(&kira, None, None).ok();
    wincan.present();

    thread::sleep(Duration::from_millis(1500));

    wincan.copy(&josh, None, None).ok();
    wincan.present();

    thread::sleep(Duration::from_millis(1500));

    wincan.copy(&alvyn, None, None).ok();
    wincan.present();

    thread::sleep(Duration::from_millis(1500));

    wincan.copy(&greg, None, None).ok();
    wincan.present();

    thread::sleep(Duration::from_millis(1500));

    wincan.copy(&jake, None, None).ok();
    wincan.present();

    thread::sleep(Duration::from_millis(1500));

    wincan.copy(&bryce, None, None).ok();
    wincan.present();

    thread::sleep(Duration::from_millis(1500));

    wincan.copy(&austin, None, None).ok();
    wincan.present();

    thread::sleep(Duration::from_millis(1500));

    wincan.copy(&evan, None, None).ok();
    wincan.present();

    thread::sleep(Duration::from_millis(1500));
}
