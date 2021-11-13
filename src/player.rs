use sdl2::render::Texture;

use crate::animation_controller::AnimController;
use crate::physics_controller::PhysicsController;
use crate::portal_controller::PortalController;
use crate::rect_collider::RectCollider;

pub struct Player<'a> {
    pub sprite_sheet: Texture<'a>,
    pub physics: PhysicsController,
    pub collider: RectCollider,
    pub anim: AnimController,
    pub portal: PortalController
}

impl Player<'_> {
    pub fn new(_sheet: Texture, _physics: PhysicsController, _collider: RectCollider, _anim: AnimController, _portal: PortalController)
               -> Player
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

    pub fn unstop(&mut self) {
        self.physics.mobilize();
        self.anim.unfreeze();
        self.portal.unfreeze();
    }

    pub fn add_collider(&mut self, collider: RectCollider, valid: bool) {
        self.physics.add_collider(collider);
        if valid {
            self.portal.add_valid_surface(collider);
        } else {
            self.portal.add_invalid_surface(collider);
        }
    }

    pub fn reset_colliders(&mut self) {
        self.physics.reset_colliders();
        self.portal.reset_surfaces();
        self.portal.close_all();
    }
}