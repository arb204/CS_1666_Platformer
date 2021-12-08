#[derive(Copy, Clone)]
pub struct RemotePlayer {
    pub player_data: (f32, f32, bool, i32, i32, u32, u32),
    pub portal_data: (f32, f32, f32),
    pub block_data: (i32, i32, bool),
    pub wand_data: (i32, i32, f32),
}