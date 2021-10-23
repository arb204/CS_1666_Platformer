pub mod player {
    use sdl2::render::Texture;
    use crate::physics_controller::physics_controller::PhysicsController;
    use crate::rect_collider::rect_collider::RectCollider;
    use crate::animation_controller::animation_controller::AnimController;
    pub struct Player<'a> {
        pub sprite_sheet: Texture<'a>,
        pub physics: PhysicsController,
        pub collider: RectCollider,
        pub anim: AnimController
    }

    impl Player<'_> {
        pub fn new<'a>(_sheet: Texture<'a>, _physics: PhysicsController, _collider: RectCollider, _anim: AnimController)
            -> Player<'a>
        {
            Player {
                sprite_sheet: _sheet,
                physics: _physics,
                collider: _collider,
                anim: _anim
            }
        }
        //getters
        pub fn physics(&self) -> PhysicsController { self.physics }
    }
}