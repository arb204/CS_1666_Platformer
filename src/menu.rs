use sdl2::render::WindowCanvas;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use std::thread;

use crate::game::show_game;

pub(crate) fn show_menu(mut wincan: WindowCanvas, mut event_pump: sdl2::EventPump)
{
    let texture_creator = wincan.texture_creator();

    let start = texture_creator.load_texture("assets/single_assets/menu.png").unwrap();

    wincan.copy(&start, None, None).ok();
    wincan.present();

    'menuloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'menuloop,
                Event::KeyDown{keycode: Some(k), ..} => {
                    match k {
                        _ => break 'menuloop,
                    }
                }
                _ => {},
            }
        }
        /*let keystate: HashSet<Keycode> = core.event_pump.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();
        if keystate.contains(&Keycode::Space) {
            show_game(core);
            // core.wincan.clear();
            // current_scene = "game"
        } */
    }

    show_game(wincan, event_pump).ok();
}