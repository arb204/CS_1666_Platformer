use std::collections::HashMap;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use crate::game::Game;


pub struct Renderer<'a> {
    wincan: WindowCanvas,
    textures: HashMap<&'a str, Texture<'a>>,
    texture_creator: TextureCreator<WindowContext>,
}

impl Renderer<'_> {
    pub fn new<'a>(wincan: WindowCanvas) -> Result<Renderer<'a>, String> {
        let texture_creator = wincan.texture_creator();
        // declare textures here
        let bluewand = texture_creator.load_texture("assets/in_game/player/wand/blue/blue_wand.png").unwrap();
        let orangewand = texture_creator.load_texture("assets/in_game/player/wand/orange/orange_wand.png").unwrap();
        let cursor = texture_creator.load_texture("assets/in_game/cursor/cursor.png").unwrap();
        let portalsprite = texture_creator.load_texture("assets/in_game/portal/portal-sprite-sheet.png").unwrap();
        let p1sprite = texture_creator.load_texture("assets/in_game/player/character/characters-sprites_condensed.png").unwrap();
        let stone_sprite = texture_creator.load_texture("assets/in_game/level/purple_floor/purple_floor_tile.png").unwrap();
        let door_sheet = texture_creator.load_texture("assets/in_game/level/door/doors_sprite_sheet.png").unwrap();
        let level_cleared_msg_sprite = texture_creator.load_texture("assets/in_game/message/level_cleared/level_cleared_msg.png").unwrap();
        let castle_bg = texture_creator.load_texture("assets/in_game/level/background/castle/castle-bg.png").unwrap();
        let nonportal_surface = texture_creator.load_texture("assets/in_game/level/brick/nonportal/stone_brick_64x64.png").unwrap();
        let portal_surface = texture_creator.load_texture("assets/in_game/level/brick/portal/portal_brick_64x64.png").unwrap();
        let portal_glass = texture_creator.load_texture("assets/in_game/level/brick/portal_glass.png").unwrap();
        let block_texture = texture_creator.load_texture("assets/in_game/block/block.png").unwrap();

        let mut textures: HashMap<&str, Texture> = HashMap::new();
        textures.insert("blue_wand", bluewand);
        textures.insert("orange_wand", orangewand);
        textures.insert("cursor", cursor);
        textures.insert("portal", portalsprite);
        textures.insert("p1sprite", p1sprite);
        textures.insert("purple_floor", stone_sprite);
        textures.insert("door_sheet", door_sheet);
        textures.insert("level_cleared_msg", level_cleared_msg_sprite);
        textures.insert("castle_bg", castle_bg);
        textures.insert("nonportal_surface", nonportal_surface);
        textures.insert("portal_surface", portal_surface);
        textures.insert("portal_glass", portal_glass);
        textures.insert("block", block_texture);


        Ok(Renderer {
            wincan,
            texture_creator,
            textures
        })
    }

    pub fn display(&mut self, game: &Game) {
        self.wincan.present();
    }

    pub fn display_start_screen(&mut self) -> Result<(), String> {
        let start_screen = self.texture_creator.load_texture("assets/out_of_game/menu/start_screen.png")?;
        self.wincan.copy(&start_screen, None, None)?;
        Ok(())
    }
}