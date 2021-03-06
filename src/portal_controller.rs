use std::time::{Duration, SystemTime};

use crate::physics_controller::PhysicsController;
use crate::rect_collider::RectCollider;

pub struct PortalController {
    wand_x: i32,
    wand_y: i32,
    potion_x: i32,
    potion_y: i32,
    wand_rotation: f32,
    pub portals: Vec<Portal>,
    pub potions: Vec<Potion>,
    should_rotate: bool,
    physics: PhysicsController,
    last_portal_used: i8,
    last_portal_time: SystemTime,
    last_teleport_time: SystemTime,
    valid_portal_surfaces: Vec<RectCollider>,
    invalid_portal_surfaces: Vec<RectCollider>,
    has_teleported_blue: i32,
    has_teleported_orange: i32
}

impl PortalController {
    pub fn new(_x: i32, _y: i32, _px: i32, _py: i32, _physics: PhysicsController, _portals: Vec<Portal>, potions: Vec<Potion>, _surfaces: Vec<RectCollider>, _inval_surfaces: Vec<RectCollider>)
        -> PortalController
    {
        PortalController {
            wand_x: _x,
            wand_y: _y,
            potion_x: _px,
            potion_y: _py,
            wand_rotation: 0.0,
            portals: _portals,
            potions: vec!(Potion::new(0), Potion::new(1)),
            should_rotate: true,
            physics: _physics,
            last_portal_used: 0,
            last_portal_time: SystemTime::now(),
            last_teleport_time: SystemTime::now(),
            valid_portal_surfaces: _surfaces,
            invalid_portal_surfaces: _inval_surfaces,
            has_teleported_blue: 0,
            has_teleported_orange: 0
        }
    }

    pub fn wand_x(&self) -> i32 { self.wand_x }
    pub fn wand_y(&self) -> i32 { self.wand_y }
    pub fn potion_x(&self) -> i32 { self.potion_x }
    pub fn potion_y(&self) -> i32 { self.potion_y }
    pub fn rotation(&self) -> f32 { self.wand_rotation }
    pub fn last_portal(&self) -> i8 { self.last_portal_used }

    pub fn add_valid_surface(&mut self, new_collider: RectCollider) {
        self.valid_portal_surfaces.push(new_collider);
    }

    pub fn add_invalid_surface(&mut self, new_collider: RectCollider) {
        self.invalid_portal_surfaces.push(new_collider);
    }

    pub fn reset_surfaces(&mut self) {
        self.valid_portal_surfaces = vec!();
        self.invalid_portal_surfaces = vec!();
    }

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

    // get_potion_state: returns a representation of where the potions are
    pub fn get_potion_state(&self) -> (Option<(f32, f32, f64)>, Option<(f32, f32, f64)>) {
        let mut state1: Option<(f32, f32, f64)> = None;
        let mut state2: Option<(f32, f32, f64)> = None;
        if self.potions[0].thrown() {
            state1 = Some((self.potions[0].x(), self.potions[0].y(), self.potions[0].rotation()));
        }
        if self.potions[1].thrown() {
            state2 = Some((self.potions[1].x(), self.potions[1].y(), self.potions[1].rotation()));
        }
        (state1, state2)
    }

    // make it so the wand doesn't rotate (like in a level complete)
    pub fn freeze(&mut self) { self.should_rotate = false; }
    pub fn unfreeze(&mut self) { self.should_rotate = true; }

    // update the physics controllers so the wand can rotate properly
    pub fn update(&mut self, newphysics: PhysicsController) {
        self.physics = newphysics;
        let potion0state = self.potions[0].update();
        let potion1state = self.potions[1].update();
        if potion0state.is_some() {
            let p0s = potion0state.unwrap();
            self.open_portal_at_point(0, (p0s.0, p0s.1), p0s.2);
        }
        if potion1state.is_some() {
            let p1s = potion1state.unwrap();
            self.open_portal_at_point(1, (p1s.0, p1s.1), p1s.2);
        }
    }

    // teleport: teleports the player to a specific portal (UNFINISHED)
    pub fn teleport(&mut self, player_collider: &mut RectCollider, player_physics: &mut PhysicsController) -> bool {
        let mut can_teleport = true;
        for p in &self.portals {
            if !p.is_open() { can_teleport = false; }
        }
        if can_teleport {
            for i in 0..self.portals.len() {
                // we can only use each portal once every 100ms
                if self.portals[i].last_used()+Duration::from_millis(500) < SystemTime::now() && player_collider.is_touching(&self.portals[i].collider()) {
                    let in_portal = i;
                    let out_portal = (i+1)%self.portals.len();
                    let in_dir = self.portals[in_portal].rotation+180.0;
                    let out_dir = (self.portals[out_portal].rotation+360.0) % 360.0;
                    let change_direction = (out_dir - in_dir + 360.0) % 360.0;
                    // exiting on a left wall
                    if out_dir == 0.0 {
                        player_physics.set_x(self.portals[out_portal].x()+30.0);
                        player_physics.set_y(self.portals[out_portal].y()+5.0);
                    }
                    // exiting on a right wall
                    else if out_dir == 180.0 {
                        player_physics.set_x(self.portals[out_portal].x()-60.0);
                        player_physics.set_y(self.portals[out_portal].y()+5.0);
                    }
                    // exiting on the ceiling
                    else if out_dir == 90.0 {
                        player_physics.set_x(self.portals[out_portal].x()+5.0);
                        player_physics.set_y(self.portals[out_portal].y()+60.0);
                    }
                    // exiting on the floor
                    else {
                        player_physics.set_x(self.portals[out_portal].x()+5.0);
                        player_physics.set_y(self.portals[out_portal].y()-90.0);
                    }
                    // conserve momentum
                    if change_direction == 90.0 || change_direction == 270.0 {
                        let old_speed = player_physics.speed();
                        if out_dir == 0.0 {
                            player_physics.set_speed(player_physics.fall_speed());
                            player_physics.set_fall_speed(-old_speed);
                        } else if out_dir == 90.0 {
                            player_physics.set_speed(-player_physics.fall_speed());
                            player_physics.set_fall_speed(old_speed);
                        } else if out_dir == 180.0 {
                            player_physics.set_speed(-player_physics.fall_speed());
                            player_physics.set_fall_speed(-old_speed);
                        } else if out_dir == 270.0 {
                            player_physics.set_speed(player_physics.fall_speed());
                            player_physics.set_fall_speed(old_speed);
                        }
                    } else if change_direction == 180.0 {
                        if out_dir == 0.0 || out_dir == 180.0 {
                            player_physics.set_speed(-player_physics.speed());
                        } else {
                            player_physics.set_fall_speed(-player_physics.fall_speed());
                        }
                    }
                    self.last_teleport_time = SystemTime::now();
                    let _ = &self.portals[out_portal].reset_last_used();
                    return true;
                }
            }
        }
        return false;
    }

    //next_rotation: returns a float indicating the angle of the next frame
    pub fn next_rotation(&mut self, mouse_x:i32, mouse_y: i32) -> f32 {
        if self.should_rotate {
            if (mouse_x as f32) > self.physics.x()+self.wand_x as f32+50.0 {
                self.wand_rotation = ((mouse_y as f32-(self.physics.y()+self.wand_y as f32+10.0))/(mouse_x as f32-(self.physics.x()+self.wand_x as f32+50.0))).atan()*57.29;
            } else {
                self.wand_rotation = 180.0 + ((mouse_y as f32-(self.physics.y()+self.wand_y as f32+10.0))/(mouse_x as f32-(self.physics.x()+self.wand_x as f32+50.0))).atan()*57.29;
            }
        }
        self.wand_rotation
    }

    // open_portal: figures out where a portal should go and opens it there
    pub fn open_portal(&mut self, index: usize) -> i32 {
        // we can only open a portal every 100ms
        if self.should_rotate && self.last_portal_time+Duration::from_millis(100) < SystemTime::now() {
            // fire two raycasts: one to determine the point where we create the portal and one to determine the angle
            let portal_point = Raycast::new(self.physics.x()+self.wand_x as f32+50.0, self.physics.y()+self.wand_y as f32+10.0, self.wand_rotation/57.29, self.all_colliders()).cast();
            let rotation_point = Raycast::new(self.physics.x()+self.wand_x as f32+50.0, self.physics.y()+self.wand_y as f32+9.0, self.wand_rotation/57.29, self.all_colliders()).cast();
            if portal_point.is_some() && rotation_point.is_some() {
                let pp = portal_point.unwrap();
                let rp = rotation_point.unwrap();
                //portals can't overlap
                if ((pp.0 - (self.portals[1-index].x()+30.0)).powf(2.0) + (pp.1 - (self.portals[1-index].y()+50.0)).powf(2.0)).powf(0.5) < 130.0 {
                    self.last_portal_used = index as i8;
                    self.last_portal_time = SystemTime::now();
                    return 0;
                }
                //how should the portal be rotated?
                let rot = if pp.1 == rp.1 {
                    //floor
                    if self.wand_rotation > 0.0 && self.wand_rotation < 180.0 {
                        -90.0
                    }
                    //ceiling
                    else {
                        90.0
                    }
                } else if pp.0 == rp.0 {
                    //left wall
                    if self.wand_rotation > 90.0 && self.wand_rotation < 270.0 {
                        0.0
                    }
                    //right wall
                    else {
                        180.0
                    }
                } else {
                    //slope
                    if self.wand_rotation > 90.0 && self.wand_rotation < 270.0 {
                        (((rp.1-pp.1)/(rp.0-pp.0)) as f32).atan()*57.29+90.0
                    }
                    else {
                        180.0+(((rp.1-pp.1)/(rp.0-pp.0)) as f32).atan()*57.29+90.0
                    }
                };
                // we hit a surface, but is it valid?
                for i in &self.invalid_portal_surfaces {
                    if i.is_touching(&RectCollider::new(pp.0 as f32-2.5, pp.1 as f32-2.5, 5.0, 5.0)) {
                        return -1;
                    }
                }
                // open the portal
                self.portals[index].open(pp.0 - 30.0, pp.1 - 50.0, rot);
            }
            self.last_portal_used = index as i8;
            self.last_portal_time = SystemTime::now();
        }
        return 1;
    }

    pub fn open_portal_at_point(&mut self, index: usize, point: (f32, f32), rot: f32) -> bool {
        if ((point.0 - (self.portals[1-index].x()+30.0)).powf(2.0) + (point.1 - (self.portals[1-index].y()+50.0)).powf(2.0)).powf(0.5) < 130.0 {
            return false;
        }
        // we hit a surface, but is it valid?
        for i in &self.invalid_portal_surfaces {
            if i.is_touching(&RectCollider::new(point.0 as f32-2.5, point.1 as f32-2.5, 5.0, 5.0)) {
                return false;
            }
        }
        self.portals[index].open(point.0-30.0, point.1-50.0, rot);
        return true;
    }

    // throw_potion: starts one potion on a trajectory
    pub fn throw_potion(&mut self, index: usize, mouse_x: i32, mouse_y: i32) {
        // start x, start y, inital x velocity, inital y velocity
        let coll = self.all_colliders();
        let sx = self.physics.x() as i32 + self.potion_x + 12;
        let sy = self.physics.y() as i32 + self.potion_y + 12;
        // the throw power is the distance between the player's position and the mouse
        let throw_power = (((sx-mouse_x) as f32).powf(2.0) + ((sy-mouse_y) as f32).powf(2.0)).powf(0.5).clamp(0.0, 15.0);
        let throw_direction = self.next_rotation(mouse_x, mouse_y)/57.29;
        let ixv = throw_power*throw_direction.cos();
        let iyv = throw_power*throw_direction.sin();
        self.potions[index].throw(sx as f32, sy as f32, ixv as f32, iyv as f32, coll);
        self.last_portal_used = index as i8;
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
    rotation: f32,
    collider: RectCollider,
    last_used: SystemTime
}

impl Portal {
    pub fn new(_color_num: i32)
        -> Portal
    {
        Portal {
            color_num: _color_num,
            x: -100.0,
            y: -100.0,
            rotation: 0.0,
            collider: RectCollider::new(-100.0, -100.0, 50.0, 90.0),
            last_used: SystemTime::now()
        }
    }

    pub fn color(&self) -> i32 { self.color_num }
    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn rotation(&self) -> f32 { self.rotation }
    pub fn collider(&self) -> RectCollider { self.collider }
    pub fn last_used(&self) -> SystemTime { self.last_used }

    pub fn reset_last_used(&mut self) { self.last_used = SystemTime::now(); }

    /*pub fn set_x(&mut self, _x: f32) { self.x = _x; }
    pub fn set_y(&mut self, _y: f32) { self.y = _y; }
    pub fn set_rotation(&mut self, _rot: f32) { self.rotation = _rot; }*/

    pub fn is_open(&self) -> bool {
        self.x > 0.0 && self.y > 0.0
    }

    // open: opens a new portal
    pub fn open(&mut self, new_x: f32, new_y: f32, new_rot: f32) {
        self.x = new_x;
        self.y = new_y;
        self.rotation = new_rot;
        self.collider.set_x(new_x+if new_rot == 0.0 || new_rot == 180.0 {10.0} else {-5.0});
        self.collider.set_y(new_y+if new_rot == 0.0 || new_rot == 180.0 {15.0} else {45.0});
        self.collider.set_width(if new_rot == 0.0 || new_rot == 180.0 {40.0} else {70.0});
        self.collider.set_height(if new_rot == 0.0 || new_rot == 180.0 {70.0} else {40.0});
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
        let allowed_offset = 5.0;
        while !has_hit && curr_x > 0.0 && curr_x < 1220.0 && curr_y > -30.0 && curr_y < 660.0 {
            curr_x += self.rotation.cos();
            curr_y += self.rotation.sin();
            for c in &self.colliders {
                if c.contains_point(curr_x, curr_y) && !has_hit {
                    has_hit = true;
                    let mut pos_reset = false;
                    //traveling left, make sure raycast hits the surface and not inside it
                    if !pos_reset && curr_x < c.x()+c.width() && curr_x > c.x()+c.width()-allowed_offset {
                        curr_x = c.x()+c.width();
                        pos_reset = true;
                    }
                    //traveling right, make sure raycast hits the surface and not inside it
                    if !pos_reset && curr_x > c.x() && curr_x < c.x()+allowed_offset {
                        curr_x = c.x();
                        pos_reset = true;
                    }
                    //traveling up, make sure raycast hits the surface and not inside it
                    if !pos_reset && curr_y < c.y()+c.height() && curr_y > c.y()+c.height()-allowed_offset {
                        curr_y = c.y()+c.height();
                        pos_reset = true;
                    }
                    //traveling down, make sure raycast hits the surface and not inside it
                    if !pos_reset && curr_y > c.y() && curr_y < c.y()+allowed_offset {
                        curr_y = c.y();
                    }
                }
                if has_hit { break; }
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

pub struct Potion {
    index: i8,
    thrown: bool,
    x: f32,
    y: f32,
    trace_x: f32,
    trace_y: f32,
    x_velocity: f32,
    y_velocity: f32,
    rotation: f64,
    collided: bool,
    trace_collided: bool,
    collision_point: (f32, f32),
    trace_point: (f32, f32),
    colliders: Vec<RectCollider>
}

impl Potion {
    pub fn new(_index: i8)
        -> Potion
    {
        Potion {
            index: _index,
            thrown: false,
            x: -100.0,
            y: -100.0,
            trace_x: -100.0,
            trace_y: -100.0,
            x_velocity: 0.0,
            y_velocity: 0.0,
            rotation: 0.0,
            collided: false,
            trace_collided: false,
            collision_point: (-100.0, -100.0),
            trace_point: (-100.0, -100.0),
            colliders: vec!()
        }
    }
    pub fn index(&self) -> i8 { self.index }
    pub fn thrown(&self) -> bool { self.thrown }
    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn rotation(&self) -> f64 { self.rotation }

    // throw: starts the potion
    pub fn throw(&mut self, _x: f32, _y: f32, _x_velocity: f32, _init_y_velocity: f32, _colliders: Vec<RectCollider>) {
        if !self.thrown {
            self.thrown = true;
            self.collided = false;
            self.trace_collided = false;
            self.x = _x;
            self.y = _y;
            self.trace_x = _x;
            self.trace_y = _y - 1.0;
            self.x_velocity = _x_velocity;
            self.y_velocity = _init_y_velocity;
            self.collision_point = (-100.0, -100.0);
            self.trace_point = (-100.0, -100.0);
            self.colliders = _colliders;
        }
    }

    // update: updates the position and checks for collisions
    pub fn update(&mut self)
        -> Option<(f32, f32, f32)>
    {
        if self.thrown {
            let allowed_offset = 10.0;
            self.y_velocity += 0.2;
            self.rotation += 5.0;
            //update the position
            if !self.collided {
                self.x += self.x_velocity;
                self.y += self.y_velocity;
                for c in &self.colliders {
                    if c.contains_point(self.x, self.y) && !self.collided {
                        self.collided = true;
                        let mut pos_reset = false;
                        if !pos_reset && self.x > c.x() && self.x < c.x()+allowed_offset {
                            // we hit the left edge of the collider
                            let old_pos = self.x;
                            self.x = c.x();
                            pos_reset = true;
                        }
                        if !pos_reset && self.x > c.x()+c.width()-allowed_offset && self.x < c.x()+c.width() {
                            // we hit the right edge of the collider
                            let old_pos = self.x;
                            self.x = c.x()+c.width();
                            pos_reset = true;
                        }
                        if !pos_reset && self.y > c.y() && self.y < c.y()+allowed_offset {
                            // we hit the top edge of the collider
                            let old_pos = self.y;
                            self.y = c.y();
                            pos_reset = true;
                        }
                        if !pos_reset && self.y > c.y()+c.height()-allowed_offset && self.y < c.y()+c.height() {
                            // we hit the bottom edge of the collider
                            let old_pos = self.y;
                            self.y = c.y()+c.height();
                            pos_reset = true;
                        }
                        self.collision_point = (self.x, self.y);
                    }
                    if self.collided { break; }
                }
            }
            //update the trace (used for calculating rotation of portal when landing)
            if !self.trace_collided {
                self.trace_x += self.x_velocity;
                self.trace_y += self.y_velocity;
                for c in &self.colliders {
                    if c.contains_point(self.trace_x, self.trace_y) && !self.trace_collided {
                        self.trace_collided = true;
                        let mut pos_reset = false;
                        if !pos_reset && self.trace_x > c.x() && self.trace_x < c.x()+allowed_offset {
                            // we hit the left edge of the collider
                            let old_pos = self.trace_x;
                            self.trace_x = c.x();
                            pos_reset = true;
                        }
                        if !pos_reset && self.trace_x > c.x()+c.width()-allowed_offset && self.trace_x < c.x()+c.width() {
                            // we hit the right edge of the collider
                            let old_pos = self.trace_x;
                            self.trace_x = c.x()+c.width();
                            pos_reset = true;
                        }
                        if !pos_reset && self.trace_y > c.y() && self.trace_y < c.y()+allowed_offset {
                            // we hit the top edge of the collider
                            let old_pos = self.trace_y;
                            self.trace_y = c.y();
                            pos_reset = true;
                        }
                        if !pos_reset && self.trace_y > c.y()+c.height()-allowed_offset && self.trace_y < c.y()+c.height() {
                            // we hit the bottom edge of the collider
                            let old_pos = self.trace_y;
                            self.trace_y = c.y()+c.height();
                            pos_reset = true;
                        }
                        self.trace_point = (self.trace_x, self.trace_y);
                    }
                    if self.trace_collided { break; }
                }
            }
            // have both the potion and trace point hit?
            if self.collided && self.trace_collided {
                let pp = self.collision_point;
                let rp = self.trace_point;
                //how should the portal be rotated?
                let rot = if pp.1 == rp.1 {
                    //floor
                    if self.y_velocity > 0.0 {
                        -90.0
                    }
                    //ceiling
                    else {
                        90.0
                    }
                } else if pp.0 == rp.0 {
                    //left wall
                    if self.x_velocity < 0.0 {
                        0.0
                    }
                    //right wall
                    else {
                        180.0
                    }
                } else {
                    //slope
                    if self.x_velocity < 0.0 {
                        (((rp.1-pp.1)/(rp.0-pp.0)) as f32).atan()*57.29+90.0
                    }
                    else {
                        180.0+(((rp.1-pp.1)/(rp.0-pp.0)) as f32).atan()*57.29+90.0
                    }
                };
                self.thrown = false;
                self.collided = false;
                self.trace_collided = false;
                let retval = Some((self.x, self.y, rot));
                self.x = -100.0;
                self.y = -100.0;
                self.x_velocity = 0.0;
                self.y_velocity = 0.0;
                return retval;
            } else {
                if self.x < 0.0 || self.x > 1280.0 || self.y < 0.0 || self.y > 720.0 {
                    // the potion is out of bounds
                    self.thrown = false;
                    self.collided = false;
                    self.trace_collided = false;
                    self.collision_point = (-100.0, -100.0);
                    self.trace_point = (-100.0, -100.0);
                    self.x = -100.0;
                    self.y = -100.0;
                    self.x_velocity = 0.0;
                    self.y_velocity = 0.0;
                }
                return None;
            }
        }
        None
    }
}
