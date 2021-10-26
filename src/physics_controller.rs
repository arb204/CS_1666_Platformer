pub mod physics_controller {
    use std::time::SystemTime;
    #[derive(Copy, Clone)]
    pub struct PhysicsController {
        x: f32,
        y: f32,
        last_x: f32,
        last_y: f32,
        speed: f32,
        max_speed: f32,
        acceleration: f32,
        jump_speed: f32,
        jumps_used: i8,
        last_jump_time: SystemTime,
        max_jumps: i8,
        stop_speed: f32,
        fall_speed: f32,
        gravity: f32,
        max_fall_speed: f32,
        is_grounded: bool,
        out_of_bounds: bool,
        can_move: bool
    }

    impl PhysicsController {
        pub fn new(_x: f32, _y:f32, _maxspeed: f32, _acceleration: f32, _jumpspeed:f32, _maxjumps: i8, _stopspeed: f32, _gravity: f32, _maxfallspeed: f32)
            -> PhysicsController
        {
            PhysicsController {
                x: _x,
                y: _y,
                last_x: 0.0,
                last_y: 0.0,
                speed: 0.0,
                max_speed: _maxspeed,
                acceleration: _acceleration,
                jump_speed: _jumpspeed,
                jumps_used: 0,
                last_jump_time: SystemTime::now(),
                max_jumps: _maxjumps,
                stop_speed: _stopspeed,
                fall_speed: 0.0,
                gravity: _gravity,
                max_fall_speed: _maxfallspeed,
                is_grounded: false,
                out_of_bounds: false,
                can_move: true
            }
        }

        //getters
        pub fn x(&self) -> f32 { self.x }
        pub fn y(&self) -> f32 { self.y }
        pub fn speed(&self) -> f32 { self.speed }
        pub fn fall_speed(&self) -> f32 { self.fall_speed }
        pub fn can_move(&self) -> bool {self.can_move}

        //setters
        pub fn set_x(&mut self, _x: f32) { self.x = _x; }
        pub fn set_y(&mut self, _y: f32) { self.y = _y; }
        pub fn set_speed(&mut self, _speed: f32) { self.speed = _speed; }
        pub fn set_fall_speed(&mut self, _fall_speed: f32) { self.fall_speed = _fall_speed; }
        pub fn set_grounded(&mut self) { self.is_grounded = true; }
        pub fn reset_jumps(&mut self) { self.jumps_used = 0; }
        pub fn immobilize(&mut self) { self.can_move = false; }

        // debug: prints out a list of the controller's current state
        pub fn debug(&mut self) {
            println!("Physics Controller status:");
            println!("\tx: {}", self.x);
            println!("\ty: {}", self.y);
            println!("\tspeed: {}", self.speed);
            println!("\tfall speed: {}", self.fall_speed);
            println!("\tjumps used: {}/{}", self.jumps_used, self.max_jumps);
            println!("\tmoving: {}", self.is_moving());
            println!("\tgrounded: {}", self.is_grounded);
        }

        // accelerate_left: accelerates the character to the left
        pub fn accelerate_left(&mut self) {
            if self.speed > -self.max_speed {
                self.speed -= self.acceleration;
            }
            if self.speed < -self.max_speed {
                self.speed = -self.max_speed;
            }
        }

        // accelerate_right: accelerates the character to the right
        pub fn accelerate_right(&mut self) {
            if self.speed < self.max_speed {
                //self.speed = self.acceleration;
                self.speed += self.acceleration;
            }
            if self.speed > self.max_speed {
                self.speed = self.max_speed;
            }
        }

        // update: manage the character's state each frame
        pub fn update(&mut self) {
            //maybe we don't want the character to move (like finishing a level)
            if self.can_move {
                //move the character if necessary
                self.x = (self.x + self.speed).clamp(0.0, 1200.0);  // replace 1200.0 later with (CAM_W - TILE_SIZE) vars
                self.y += self.fall_speed;

                // decelerate the character
                if self.speed > 0.0 {
                    self.speed -= self.stop_speed;
                    if self.speed < 0.0 { self.speed = 0.0; }
                } else if self.speed < 0.0 {
                    self.speed += self.stop_speed;
                    if self.speed > 0.0 { self.speed = 0.0; }
                }

                //simulate gravity
                if !self.is_grounded && self.fall_speed < self.max_fall_speed {
                    self.fall_speed += self.gravity;
                }

                //reset jumps if we're on the ground
                if self.is_grounded {
                    self.jumps_used = 0;
                }

                //check if we're out of bounds and correct if needed
                if self.out_of_bounds {
                    self.x = self.last_x;
                    self.y = self.last_y;
                    self.out_of_bounds = false;
                } else {
                    self.last_x = self.x;
                    self.last_y = self.y;
                }
            }
        }

        //jump: if we have jumps left, give ourselves a boost upwards. this is so we can support multiple jumps if we need
        pub fn jump(&mut self) {
            // the time comparison here is to prevent jumps from occurring on successive frames, which would be frustrating to players
            if /*SystemTime::now().duration_since(self.last_jump_time).ok().Some > 100 &&*/ self.jumps_used < self.max_jumps {
                self.jumps_used += 1;
                self.fall_speed = -self.jump_speed;
                self.last_jump_time = SystemTime::now();
                self.is_grounded = false;
            }
        }

        //is_moving: returns true if our position was updated last frame, otherwise returns false
        pub fn is_moving(&mut self) -> bool {
            self.speed != 0.0 && self.fall_speed != 0.0
        }

        //out_of_bounds: causes the controller to go out of bounds, reverting to a legal position in the next frame.
        pub fn out_of_bounds(&mut self) {
            self.out_of_bounds = true;
        }
    }
}
