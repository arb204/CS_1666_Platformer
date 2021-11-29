use std::collections::HashMap;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use crate::game::Game;


pub struct Renderer<'a> {
    wincan: WindowCanvas,
    textures: Option<HashMap<String, Texture<'a>>>,
    texture_creator: Option<TextureCreator<WindowContext>>,
}

impl Renderer<'_> {
    pub fn new(wincan: WindowCanvas) -> Result<Renderer<'static>, String> {
        Ok(Renderer {
            wincan,
            texture_creator: None,
            textures: None
        })
    }

    pub fn init(&mut self) -> Result<(), String> {
        self.texture_creator = Some(self.wincan.texture_creator());
        // declare textures here
        let start_screen = self.texture_creator.as_ref().unwrap().load_texture("assets/out_of_game/menu/start_screen.png")?;
        let bluewand = self.texture_creator.as_ref().unwrap()
            .load_texture("assets/in_game/player/wand/blue/blue_wand.png")?;
        let orangewand = self.texture_creator.as_ref().unwrap().load_texture("assets/in_game/player/wand/orange/orange_wand.png")?;
        let cursor = self.texture_creator.as_ref().unwrap().load_texture("assets/in_game/cursor/cursor.png")?;
        let portalsprite = self.texture_creator.as_ref().unwrap().load_texture("assets/in_game/portal/portal-sprite-sheet.png")?;
        let p1sprite = self.texture_creator.as_ref().unwrap().load_texture("assets/in_game/player/character/characters-sprites_condensed.png")?;
        let stone_sprite = self.texture_creator.as_ref().unwrap().load_texture("assets/in_game/level/purple_floor/purple_floor_tile.png")?;
        let door_sheet = self.texture_creator.as_ref().unwrap().load_texture("assets/in_game/level/door/doors_sprite_sheet.png")?;
        let level_cleared_msg_sprite = self.texture_creator.as_ref().unwrap().load_texture("assets/in_game/message/level_cleared/level_cleared_msg.png")?;
        let castle_bg = self.texture_creator.as_ref().unwrap().load_texture("assets/in_game/level/background/castle/castle-bg.png")?;
        let nonportal_surface = self.texture_creator.as_ref().unwrap().load_texture("assets/in_game/level/brick/nonportal/stone_brick_64x64.png")?;
        let portal_surface = self.texture_creator.as_ref().unwrap().load_texture("assets/in_game/level/brick/portal/portal_brick_64x64.png")?;
        let portal_glass = self.texture_creator.as_ref().unwrap().load_texture("assets/in_game/level/brick/portal_glass.png")?;
        let block_texture = self.texture_creator.as_ref().unwrap().load_texture("assets/in_game/block/block.png")?;

        let mut textures: HashMap<String, Texture> = HashMap::new();
        textures.insert(String::from("start_screen"), start_screen);
        textures.insert(String::from("blue_wand"), bluewand);
        textures.insert(String::from("orange_wand"), orangewand);
        textures.insert(String::from("cursor"), cursor);
        textures.insert(String::from("portal"), portalsprite);
        textures.insert(String::from("p1sprite"), p1sprite);
        textures.insert(String::from("purple_floor"), stone_sprite);
        textures.insert(String::from("door_sheet"), door_sheet);
        textures.insert(String::from("level_cleared_msg"), level_cleared_msg_sprite);
        textures.insert(String::from("castle_bg"), castle_bg);
        textures.insert(String::from("nonportal_surface"), nonportal_surface);
        textures.insert(String::from("portal_surface"), portal_surface);
        textures.insert(String::from("portal_glass"), portal_glass);
        textures.insert(String::from("block"), block_texture);

        self.textures = Some(textures);
        Ok(())
    }

    pub fn display(&mut self, game: &Game) {
        self.wincan.present();
    }

    pub fn display_start_screen(&mut self) -> Result<(), String> {
        match self.textures {
            None => {Err(String::from("Renderer not initialized"))}
            Some(_) => {
                self.wincan.copy(&self.textures.unwrap()["start_screen"], None, None)?;
                Ok(())
            }
        }
    }
}