use std::convert::TryInto;

use sdl2::rect::Rect;
use crate::game::physics_controller::PhysicsController;

pub struct AnimController {
    columns: i32,
    width: i32,
    height: i32,
    animations: Vec<Anim>,
    previous_frame: i32,
    frames_frozen: i32,
    should_animate: bool
}

impl AnimController {
    pub fn new(_columns: i32, _width:i32, _height: i32, _animations: Vec<Anim>)
        -> AnimController
    {
        AnimController {
            columns: _columns,
            width: _width,
            height: _height,
            animations: _animations,
            previous_frame: 0,
            frames_frozen: 0,
            should_animate: true
        }
    }

    // make it so the character isn't animated (like in a level complete)
    pub fn freeze(&mut self) { self.should_animate = false; }
    pub fn unfreeze(&mut self) { self.should_animate = true; }

    // update the physics controllers so the animations know what to do
    pub fn update(&mut self, newphysics: PhysicsController) {
        for anim in self.animations.iter_mut() {
            anim.condition.update(newphysics.clone());
        }
    }

    //next_anim: returns a rect representing the next frame to be drawn
    pub fn next_anim(&mut self) -> Rect {
        return if self.should_animate {
            let valid_animations = self.animations.iter().filter(|a| a.current_priority() >= 0).collect::<Vec<&Anim>>();
            let mut max_priority_anim = valid_animations[0];
            // find which animation has the highest priority
            for anim in valid_animations {
                if anim.current_priority() > max_priority_anim.current_priority() {
                    max_priority_anim = anim;
                }
            }
            let mut new_frame = self.previous_frame;
            if max_priority_anim.frames().contains(&self.previous_frame) {
                // do we need to freeze this animation for more frames? if so, just wait
                if self.frames_frozen < max_priority_anim.frame_duration(self.previous_frame) {
                    self.frames_frozen += 1;
                } else {
                    // if we are ready to draw the next frame, draw the next one in the sequence
                    new_frame = max_priority_anim.frames()[(max_priority_anim.frames().iter().position(|&f| f == self.previous_frame).unwrap() + 1) % max_priority_anim.frames().len()];
                    self.frames_frozen = 0;
                }
            } else {
                // if we weren't using this animation, switch to the first frame in that animation
                new_frame = max_priority_anim.frames()[0];
                self.frames_frozen = 0;
            }
            self.previous_frame = new_frame;
            // calculate where in the sprite sheet this frame is and return it
            Rect::new((new_frame % self.columns) * self.width, (new_frame / self.columns) * self.height, self.width as u32, self.height as u32)
        } else {
            Rect::new((self.previous_frame % self.columns) * self.width, (self.previous_frame / self.columns) * self.height, self.width as u32, self.height as u32)
        }
    }
}

pub struct Anim {
    frames: Vec<i32>,
    durations: Vec<i32>,
    condition: Condition,
}

impl Anim {
    pub fn new(_frames: Vec<i32>, _durations: Vec<i32>, _condition: Condition)
        -> Anim
    {
        Anim {
            frames: _frames,
            durations: _durations,
            condition: _condition
        }
    }
    //getters
    pub fn frames(&self) -> &Vec<i32> { &self.frames }

    // current_priority(): returns its animation priority if it is ready to run, returns -1 otherwise
    pub fn current_priority(&self) -> i32 {
        if self.condition.is_met() { return self.condition.priority(); }
        -1
    }

    // frame_index(): returns the index of a specified frame
    pub fn frame_index(&self, frame_num: i32) -> i32 {
        self.frames.iter().position(|&f| f == frame_num).unwrap().try_into().unwrap()
    }

    // frame_duration(): returns the duration of a specified frame
    pub fn frame_duration(&self, frame_num: i32) -> i32 {
        self.durations[self.frame_index(frame_num) as usize]
    }
}

pub struct Condition {
    condition: String,
    priority: i32,
    physics: PhysicsController
}

impl Condition {

    pub fn new(_condition: String, _priority: i32, _physics: PhysicsController)
        -> Condition
    {
        Condition {
            condition: _condition,
            priority: _priority,
            physics: _physics
        }
    }

    //getters
    pub fn priority(&self) -> i32 { self.priority }

    pub fn update(&mut self, newphysics: PhysicsController) {
                                                          self.physics = newphysics;
                                                                                    }

    // is_met(): returns true if the string condition is true after being parsed, false otherwise
    pub fn is_met(&self) -> bool {
        // split the string into three sections: the keyword of the field we want to check,
        // the comparator we want to use, and the value we want to compare it to
        if self.condition == "true" { return true; }
        let split_str = self.condition.split(" ").collect::<Vec<&str>>();
        //println!("split condition: {} {} {}", split_str[0], split_str[1], split_str[2]);
        let field = split_str[0];
        let comparator = split_str[1];
        let value: f32 = split_str[2].parse().unwrap();
        if field == "x" {
            if comparator == ">" { return self.physics.x() > value; }
            if comparator == "<" { return self.physics.x() < value; }
            if comparator == "<=" { return self.physics.x() <= value; }
            if comparator == ">=" { return self.physics.x() >= value; }
            if comparator == "=" { return self.physics.x() == value; }
            if comparator == "!=" { return self.physics.x() != value; }
        }
        if field == "y" {
            if comparator == ">" { return self.physics.y() > value; }
            if comparator == "<" { return self.physics.y() < value; }
            if comparator == "<=" { return self.physics.y() <= value; }
            if comparator == ">=" { return self.physics.y() >= value; }
            if comparator == "=" { return self.physics.y() == value; }
            if comparator == "!=" { return self.physics.y() != value; }
        }
        if field == "speed" {
            if comparator == ">" { return self.physics.speed() > value; }
            if comparator == "<" { return self.physics.speed() < value; }
            if comparator == "<=" { return self.physics.speed() <= value; }
            if comparator == ">=" { return self.physics.speed() >= value; }
            if comparator == "=" { return self.physics.speed() == value; }
            if comparator == "!=" { return self.physics.speed() != value; }
        }
        if field == "fallspeed" {
            if comparator == ">" { return self.physics.fall_speed() > value; }
            if comparator == "<" { return self.physics.fall_speed() < value; }
            if comparator == "<=" { return self.physics.fall_speed() <= value; }
            if comparator == ">=" { return self.physics.fall_speed() >= value; }
            if comparator == "=" { return self.physics.fall_speed() == value; }
            if comparator == "!=" { return self.physics.fall_speed() != value; }
        }
        false
    }
}