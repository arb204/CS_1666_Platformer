pub mod player {
    use sdl2::render::Texture;
    use crate::physics_controller::physics_controller::PhysicsController;
    use crate::rect_collider::rect_collider::RectCollider;
    use crate::animation_controller::animation_controller::AnimController;
    use crate::portal_controller::portal_controller::PortalController;
    pub struct Player<'a> {
        pub sprite_sheet: Texture<'a>,
        pub physics: PhysicsController,
        pub collider: RectCollider,
        pub anim: AnimController,
        pub portal: PortalController
    }

    impl Player<'_> {
        pub fn new<'a>(_sheet: Texture<'a>, _physics: PhysicsController, _collider: RectCollider, _anim: AnimController, _portal: PortalController)
            -> Player<'a>
        {
            Player {
                sprite_sheet: _sheet,
                physics: _physics,
                collider: _collider,
                anim: _anim,
                portal: _portal
            }
        }

        // update: handle all the updates we need
        pub fn update(&mut self) {
            self.physics.update();
            self.collider.update(&self.physics.clone());
            self.anim.update(self.physics.clone());
            self.portal.update(self.physics.clone());
        }

        // stop: freeze the player in place
        pub fn stop(&mut self) {
            self.physics.immobilize();
            self.anim.freeze();
            self.portal.freeze();
        }
    }
}
