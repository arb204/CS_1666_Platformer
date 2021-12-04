use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseUtil;
use sdl2::render::WindowCanvas;

use crate::game;
use crate::networking::Network;

pub(crate) fn show_menu(mut wincan: WindowCanvas, mut event_pump: sdl2::EventPump, mouse: MouseUtil, network: Option<Network>)
{
    let texture_creator = wincan.texture_creator();

    let start = texture_creator.load_texture("assets/out_of_game/menu/start_screen.png").unwrap();

    wincan.copy(&start, None, None).ok();
    wincan.present();

    'menu_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'menu_loop,
                Event::KeyDown{keycode: Some(k), ..} => {
                    match k {
                        _ => break 'menu_loop,
                    }
                }
                _ => {},
            }
        }
    }
    game::run(wincan, event_pump, mouse, network).ok();
}