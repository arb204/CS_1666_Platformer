//ObjectController: controls physics and collision for carryable objects (UNFINISHED)
use crate::rect_collider::RectCollider;
use crate::player::Player;
pub struct ObjectController {
    collider: RectCollider,
    obstacles: Vec<RectCollider>,
    carried: bool,
    in_air: bool,
    new_level: bool,
    fall_speed: f32,
    pub offset: (f32, f32),
    start_x: f32,
    start_y: f32,
}

impl ObjectController {
    pub fn new(_collider: RectCollider) -> ObjectController
    {
        ObjectController {
            collider: _collider,
            obstacles: vec!(),
            carried: false,
            in_air: false,
            new_level: false,
            fall_speed: 0.0,
            offset: (0.0, 0.0),
            start_x: 0.0,
            start_y: 0.0,
        }
    }

    pub fn x(&self) -> i32 { self.collider.x() as i32}
    pub fn y(&self) -> i32 { self.collider.y() as i32}
    pub fn carried(&self) -> bool { self.carried }
    pub fn in_air(&self) -> bool { self.in_air }
    pub fn new_level(&self) -> bool { self.new_level }
    pub fn fall_speed(&self) -> f32 { self.fall_speed }
    pub fn collider(&self) -> RectCollider { self.collider }

    pub fn reset_colliders(&mut self) { self.obstacles = vec!(); }
    pub fn add_collider(&mut self, wall: RectCollider) {
        self.obstacles.push(wall);
    }

    pub fn picked_up(&mut self, player: &Player) {
        self.carried = true;
        self.in_air = true;
        self.collider.set_y((self.y() - 20) as f32);
        self.offset = get_offset(self.collider(), player.collider);
    }

    pub fn put_down(&mut self) {
        self.carried = false;
    }

    pub fn respawn(&mut self) {
        self.new_level = true;
    }

    pub fn set_start_pos(&mut self, x: f32, y: f32) {
        self.start_x = x;
        self.start_y = y;
    }

    pub fn update(&mut self, player: &Player) {
        if self.new_level {
            self.new_level = false;
            self.carried = false;
            self.in_air = true;
            self.collider.set_x(self.start_x);
            self.collider.set_y(self.start_y);
        }
        
        if self.carried {
            self.collider.set_x((player.collider.x()-self.offset.0) as f32);
            self.collider.set_y((player.collider.y()-self.offset.1) as f32);
        }
        else if self.in_air {
            self.fall_speed += 1.0;
            let predict = RectCollider::new(self.x() as f32, self.y() as f32 + self.fall_speed(), self.collider.width(), self.collider.height());
            let mut ground = 721.0;
            for wall in &self.obstacles {
                if predict.is_touching(&wall) { ground = wall.y(); }
            }
            if ground < 721.0 {
                self.collider.set_y(ground-(64/2) as f32);
                self.in_air = false;
                self.fall_speed = 0.0;
            }
            else {
                self.collider.set_y(self.y() as f32 + self.fall_speed());
            }
        }
    }
}

pub fn get_offset(inner: RectCollider, outer: RectCollider) -> (f32, f32) {
    let y = outer.y() - inner.y();
    let x = outer.x() - inner.x();
    (x as f32, y as f32)
}
