use sdl2::render::WindowCanvas;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::mouse::MouseUtil;
use std::thread;

use crate::game::show_game;
use crate::networking::networking::NetworkingMode;

pub(crate) fn show_menu(mut wincan: WindowCanvas, mut event_pump: sdl2::EventPump, mouse: MouseUtil)
{
    let texture_creator = wincan.texture_creator();

    let start = texture_creator.load_texture("assets/single_assets/menu.png").unwrap();

    let mut network_mode: NetworkingMode = NetworkingMode::Send;

    wincan.copy(&start, None, None).ok();
    wincan.present();

    'menuloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'menuloop,
                /*
                Event::KeyDown{keycode: Some(Keycode::M), ..} => {
                    network_mode = NetworkingMode::Recieve;
                    break 'menuloop;
                }
                */
                Event::KeyDown{keycode: Some(k), ..} => {
                    match k {
                        Keycode::M => {
                            network_mode = NetworkingMode::Recieve;
                            break 'menuloop;
                        }
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
    match &network_mode {
        NetworkingMode::Send => print!("SENDING"),
        NetworkingMode::Recieve => print!("RECIEVING"),
    }

    show_game(wincan, event_pump, mouse, network_mode).ok();
}