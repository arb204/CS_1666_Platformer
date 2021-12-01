use sdl2::render::Texture;

use crate::animation_controller::AnimController;
use crate::physics_controller::PhysicsController;
use crate::portal_controller::PortalController;
use crate::rect_collider::RectCollider;
use crate::plate_controller::PlateController;

pub struct Player {
    pub physics: PhysicsController,
    pub collider: RectCollider,
    pub anim: AnimController,
    pub portal: PortalController,
    dead: bool
}

impl Player {
    pub fn new(_physics: PhysicsController, _collider: RectCollider, _anim: AnimController, _portal: PortalController)
               -> Player
    {
        Player {
            physics: _physics,
            collider: _collider,
            anim: _anim,
            portal: _portal,
            dead: false
        }
    }
    pub fn is_dead(&self) -> bool { self.dead }

    // update: handle all the updates we need
    pub fn update(&mut self, platecon: PlateController) {
        self.physics.update(platecon);
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

    pub fn add_collider(&mut self, collider: RectCollider, block_type: &str) {
        self.physics.add_collider(collider);
        if block_type == "portalblock" {
            self.portal.add_valid_surface(collider);
        } else if block_type == "nonportalblock" {
            self.portal.add_invalid_surface(collider);
        }
    }

    pub fn reset_colliders(&mut self) {
        self.physics.reset_colliders();
        self.portal.reset_surfaces();
        self.portal.close_all();
    }

    // kill: kill the player
    pub fn kill(&mut self) {
        self.portal.close_all();
        self.dead = true;
        self.physics.set_x(-300.0);
        self.physics.set_y(-300.0);
    }

    // respawn: respawn the player
    pub fn respawn(&mut self) {
        self.dead = false;
        self.physics.respawn();
        self.physics.set_speed(0.0);
    }
}
