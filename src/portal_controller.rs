pub mod portal_controller {
    use crate::physics_controller::physics_controller::PhysicsController;
    use crate::rect_collider::rect_collider::RectCollider;
    use std::time::{Duration, SystemTime};
    pub struct PortalController {
        wand_x: i32,
        wand_y: i32,
        wand_rotation: f32,
        pub portals: Vec<Portal>,
        portal_colliders: Vec<RectCollider>,
        should_rotate: bool,
        physics: PhysicsController,
        last_portal_used: i8,
        last_portal_time: SystemTime,
        valid_portal_surfaces: Vec<RectCollider>,
        invalid_portal_surfaces: Vec<RectCollider>,
        has_teleported_blue: i32,
        has_teleported_orange: i32
    }

    impl PortalController {
        pub fn new(_x: i32, _y: i32, _physics: PhysicsController, _portals: Vec<Portal>, _portal_colliders: Vec<RectCollider>, _surfaces: Vec<RectCollider>, _inval_surfaces: Vec<RectCollider>)
            -> PortalController
        {
            PortalController {
                wand_x: _x,
                wand_y: _y,
                wand_rotation: 0.0,
                portals: _portals,
                portal_colliders: _portal_colliders,
                should_rotate: true,
                physics: _physics,
                last_portal_used: 0,
                last_portal_time: SystemTime::now(),
                valid_portal_surfaces: _surfaces,
                invalid_portal_surfaces: _inval_surfaces,
                has_teleported_blue: 0,
                has_teleported_orange: 0
            }
        }

        pub fn wand_x(&self) -> i32 { self.wand_x }
        pub fn wand_y(&self) -> i32 { self.wand_y }
        pub fn last_portal(&self) -> i8 { self.last_portal_used }

        pub fn all_colliders(&self) -> Vec<RectCollider> {
            let mut return_vec: Vec<RectCollider> = vec!();
            for v in &self.valid_portal_surfaces {
                return_vec.push(*v);
            }
            for i in &self.invalid_portal_surfaces {
                return_vec.push(*i);
            }
            return_vec
        }

        // make it so the wand doesn't rotate (like in a level complete)
        pub fn freeze(&mut self) { self.should_rotate = false; }

        // update the physics controllers so the wand can rotate properly
        pub fn update(&mut self, newphysics: PhysicsController) {
            self.physics = newphysics;
        }

        // teleport: teleports the player to a specific portal (UNFINISHED)
        pub fn teleport(&mut self, playerCollider: &RectCollider, playerPhysics: &mut PhysicsController) {
            // test to see if the player is touching the portal
            if playerCollider.is_touching(&self.portal_colliders[0]) && self.has_teleported_blue == 0 {
                playerPhysics.set_x(self.portal_colliders[1].x());
                playerPhysics.set_y(self.portal_colliders[1].y() + 40.0);
                self.has_teleported_orange = 1;
            }
            if playerCollider.is_touching(&self.portal_colliders[1]) && self.has_teleported_orange == 0 {
                playerPhysics.set_x(self.portal_colliders[0].x());
                playerPhysics.set_y(self.portal_colliders[0].y() + 40.0);
                self.has_teleported_blue = 1;
            }
            // makes sure player doesn't rapidly teleport back and forth
            if !playerCollider.is_touching(&self.portal_colliders[0]) && !playerCollider.is_touching(&self.portal_colliders[1]) {
                self.has_teleported_orange = 0;
                self.has_teleported_blue = 0;
            }
        }

        //next_rotation: returns a float indicating the angle of the next frame
        pub fn next_rotation(&mut self, mouse_x:i32, mouse_y: i32) -> f32 {
            if self.should_rotate {
                if (mouse_x as f32) > self.physics.x()+self.wand_x as f32 {
                    self.wand_rotation = ((mouse_y as f32-(self.physics.y()+self.wand_y as f32))/(mouse_x as f32-(self.physics.x()+self.wand_x as f32))).atan()*57.29;
                } else {
                    self.wand_rotation = 180.0 + ((mouse_y as f32-(self.physics.y()+self.wand_y as f32))/(mouse_x as f32-(self.physics.x()+self.wand_x as f32))).atan()*57.29;
                }
            }
            self.wand_rotation
        }

        // open_portal: figures out where a portal should go and opens it there
        pub fn open_portal(&mut self, index: usize) {
            // we can only open a portal every 100ms
            if self.should_rotate && self.last_portal_time+Duration::from_millis(100) < SystemTime::now() {
                // fire two raycasts: one to determine the point where we create the portal and one to determine the angle
                let portal_point = Raycast::new(self.physics.x()+self.wand_x as f32, self.physics.y()+self.wand_y as f32, self.wand_rotation/57.29, self.all_colliders()).cast();
                let rotation_point = Raycast::new(self.physics.x()+self.wand_x as f32, self.physics.y()+self.wand_y as f32-1.0, self.wand_rotation/57.29, self.all_colliders()).cast();
                if portal_point.is_some() && rotation_point.is_some() {
                    let pp = portal_point.unwrap();
                    let rp = rotation_point.unwrap();
                    let rot = if rp.0 == pp.0 { 0.0 } else if rp.1 > pp.1-1.0 && rp.1 < pp.1+1.0 { 90.0 } else { (((rp.1-pp.1)/(rp.0-pp.0)) as f32).atan()*57.29+90.0 };
                    // we hit a surface, but is it valid?
                    for i in &self.invalid_portal_surfaces {
                        if i.is_touching(&RectCollider::new(pp.0 as f32, pp.1 as f32, 5.0, 5.0)) {
                            return;
                        }
                    }
                    self.portals[index].open(pp.0 - 30.0, pp.1 - 50.0, rot);
                    self.portal_colliders[index].set_x(pp.0 - 30.0);
                    self.portal_colliders[index].set_y(pp.1 - 50.0);
                }
                self.last_portal_used = index as i8;
                self.last_portal_time = SystemTime::now();
            }
        }

        // close_all: closes all open portals
        pub fn close_all(&mut self) {
            for i in 0..self.portals.len() {
                self.portals[i].close();
            }
        }
    }

    pub struct Portal {
        color_num: i32,
        x: f32,
        y: f32,
        rotation: f32
    }

    impl Portal {
        pub fn new(_color_num: i32)
            -> Portal
        {
            Portal {
                color_num: _color_num,
                x: -100.0,
                y: -100.0,
                rotation: 0.0
            }
        }

        pub fn color(&self) -> i32 { self.color_num }
        pub fn x(&self) -> f32{ self.x }
        pub fn y(&self) -> f32{ self.y }
        pub fn rotation(&self) -> f32{ self.rotation }

        /*pub fn set_x(&mut self, _x: f32) { self.x = _x; }
        pub fn set_y(&mut self, _y: f32) { self.y = _y; }
        pub fn set_rotation(&mut self, _rot: f32) { self.rotation = _rot; }*/

        // open: opens a new portal
        pub fn open(&mut self, new_x: f32, new_y: f32, new_rot: f32) {
            self.x = new_x;
            self.y = new_y;
            self.rotation = new_rot;
        }

        // close: closes a portal by moving it offscreen
        pub fn close(&mut self) {
            self.x = -100.0;
            self.y = -100.0;
            self.rotation = 0.0;
        }
    }

    pub struct Raycast {
        start_x: f32,
        start_y: f32,
        rotation: f32,
        colliders: Vec<RectCollider>
    }

    impl Raycast {
        pub fn new(_x: f32, _y: f32, _rot: f32, _colliders: Vec<RectCollider>)
            -> Raycast
        {
            Raycast {
                start_x: _x,
                start_y: _y,
                rotation: _rot,
                colliders: _colliders
            }
        }

        // cast until we hit a collider
        pub fn cast(&mut self) -> Option<(f32, f32)> {
            let mut curr_x = self.start_x;
            let mut curr_y = self.start_y;
            let mut has_hit = false;
            while !has_hit && curr_x > 0.0 && curr_x < 1220.0 && curr_y > -30.0 && curr_y < 660.0 {
                curr_x += self.rotation.cos();
                curr_y += self.rotation.sin();
                for c in &self.colliders {
                    if c.contains_point(curr_x, curr_y) {
                        has_hit = true;
                    }
                }
            }
            if has_hit {
                Some((curr_x, curr_y))
            } else {
                None
            }
        }

        // try to cast through a specific point
        /*pub fn cast_through(&mut self, target_x: f32, target_y: f32) -> Option<Point> {
            self.rotation = if target_x > self.start_x {
                ((target_y-self.start_y)/(target_x-self.start_x)).atan()*57.29
            } else {
                180.0+((target_y-self.start_y)/(target_x-self.start_x)).atan()*57.29
            };
            self.cast()
        }*/
    }
}
