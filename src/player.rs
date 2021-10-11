pub mod player {
    use sdl2::render::Texture;
    use crate::physics_controller::physics_controller::PhysicsController;
    use crate::rect_collider::rect_collider::RectCollider;
    pub struct Player<'a> {
        sprite_sheet: Texture<'a>,
        physics: PhysicsController,
        collider: RectCollider
    }

    impl Player<'_> {
        fn new(_sheet: Texture, _physics: PhysicsController, _collider: RectCollider)
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