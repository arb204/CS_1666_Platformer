pub mod physics_controller {
    use std::time::SystemTime;
    pub struct PhysicsController {
        ID: String,
        x: f32,
        y: f32,
        last_x: f32,
        last_y: f32,
        speed: f32,
        MAX_SPEED: f32,
        ACCELERATION: f32,
        JUMP_SPEED: f32,
        jumps_used: i8,
        last_jump_time: SystemTime,
        max_jumps: i8,
        STOP_SPEED: f32,
        fall_speed: f32,
        GRAVITY: f32,
        max_fall_speed: f32,
        is_grounded: bool,
        out_of_bounds: bool
    }

    impl PhysicsController {
        fn new(_id: String, _x: f32, _y:f32, _maxspeed: f32, _acceleration: f32, _jumpspeed:f32, _maxjumps: i8, _stopspeed: f32, _gravity: f32, _maxfallspeed: f32)
            -> PhysicsController
        {
            PhysicsController {
                ID: _id,
                x: _x,
                y: _y,
                last_x: 0.0,
                last_y: 0.0,
                speed: 0.0,
                MAX_SPEED: _maxspeed,
                ACCELERATION: _acceleration,
                JUMP_SPEED: _jumpspeed,
                jumps_used: 0,
                last_jump_time: SystemTime::now(),
                max_jumps: _maxjumps,
                STOP_SPEED: _stopspeed,
                fall_speed: 0.0,
                GRAVITY: _gravity,
                max_fall_speed: _maxfallspeed,
                is_grounded: false,
                out_of_bounds: false
            }
        }

        // debug: prints out a list of the controller's current state
        fn debug(&mut self) {
            println!("Physics Controller'{}' status:", self.ID);
            println!("\tx: {}", self.x);
            println!("\ty: {}", self.y);
            println!("\tspeed: {}", self.speed);
            println!("\tspeed: {}", self.fall_speed);
            println!("\tmoving: {}", self.is_moving());
            println!("\tgrounded: {}", self.is_grounded);
            println!("\tout of bounds: {}", self.out_of_bounds);
        }

        // accelerate_left: accelerates the character to the left
        fn accelerate_left(&mut self) {
            if self.speed > -self.MAX_SPEED {
                self.speed -= self.ACCELERATION;
            }
            if self.speed < -self.MAX_SPEED {
                self.speed = -self.MAX_SPEED;
            }
        }

        // accelerate_right: accelerates the character to the right
        fn accelerate_right(&mut self) {
            if self.speed < self.MAX_SPEED {
                self.speed += self.ACCELERATION;
            }
            if self.speed > self.MAX_SPEED {
                self.speed = self.MAX_SPEED;
            }
        }

        // update: manage the character's state each frame
        fn update(&mut self) {
            //move the character if necessary
            self.x += self.speed;
            self.y += self.fall_speed;

            // decelerate the character
            if self.speed > 0.0 {
                self.speed -= self.STOP_SPEED;
                if self.speed < 0.0 { self.speed = 0.0; }
            } else if self.speed < 0.0 {
                self.speed += self.STOP_SPEED;
                if self.speed > 0.0 { self.speed = 0.0; }
            }

            //simulate gravity
            if !self.is_grounded && self.fall_speed < self.max_fall_speed {
                self.fall_speed += self.GRAVITY;
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

        //jump: if we have jumps left, give ourselves a boost upwards. this is so we can support multiple jumps if we need
        fn jump(&mut self) {
            // the time comparison here is to prevent jumps from occurring on successive frames, which would be frustrating to players
            //if SystemTime::now().duration_since(self.last_jump_time).ok().Some > 100 && self.jumps_used < self.max_jumps {
                self.jumps_used += 1;
                self.fall_speed = -self.JUMP_SPEED;
                self.last_jump_time = SystemTime::now();
            //}
        }

        //is_moving: returns true if our position was updated last frame, otherwise returns false
        fn is_moving(&mut self) -> bool {
            self.speed != 0.0 && self.fall_speed != 0.0
        }
    }
}