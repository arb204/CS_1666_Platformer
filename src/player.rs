pub mod player {
    use sdl2::render::Texture;
    use crate::physics_controller::physics_controller::PhysicsController;
    use crate::rect_collider::rect_collider::RectCollider;
    pub struct Player<'a> {
        pub sprite_sheet: Texture<'a>,
        pub physics: PhysicsController,
        pub collider: RectCollider
    }

    impl Player<'_> {
        pub fn new(_sheet: Texture, _physics: PhysicsController, _collider: RectCollider)
            -> Player
        {
            Player {
                sprite_sheet: _sheet,
                physics: _physics,
                collider: _collider
            }
        }
    }
}
