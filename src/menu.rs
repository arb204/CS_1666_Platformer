use sdl2::render::WindowCanvas;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::mouse::MouseUtil;


use crate::game::show_game;
use crate::networking::NetworkingMode;

pub(crate) fn show_menu(mut wincan: WindowCanvas, mut event_pump: sdl2::EventPump, mouse: MouseUtil, network_mode: NetworkingMode)
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
                        // Keycode::M => {
                        //     network_mode = NetworkingMode::Receive;
                        //     break 'menuloop
                        // }
                        _ => break 'menuloop,
                    }
                }
                _ => {},
            }
        }
    }
    // match &network_mode {
    //     NetworkingMode::Send => print!("SENDING"),
    //     NetworkingMode::Receive => print!("RECEIVING"),
    // }
    show_game(wincan, event_pump, mouse, network_mode).ok();
}