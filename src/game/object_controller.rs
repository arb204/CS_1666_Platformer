//ObjectController: controls physics and collision for carryable objects (UNFINISHED)
use crate::game::rect_collider::RectCollider;
use crate::game::player::Player;
pub struct ObjectController {
    collider: RectCollider,
    carried: bool,
    in_air: bool,
    fall_speed: f32,
    pub offset: (f32, f32),
}

impl ObjectController {
    pub fn new(_collider: RectCollider) -> ObjectController
    {
        ObjectController {
            collider: _collider,
            carried: false,
            in_air: false,
            fall_speed: 0.0,
            offset: (0.0, 0.0)
        }
    }

    pub fn x(&self) -> i32 { self.collider.x() as i32}
    pub fn y(&self) -> i32 { self.collider.y() as i32}
    pub fn carried(&self) -> bool { self.carried }
    pub fn in_air(&self) -> bool { self.in_air }
    pub fn fall_speed(&self) -> f32 { self.fall_speed }
    pub fn collider(&self) -> RectCollider { self.collider }

    pub fn picked_up(&mut self, player: &Player) {
        self.carried = true;
        self.in_air = true;
        self.collider.set_y((self.y() - 20) as f32);
        self.offset = get_offset(self.collider(), player.collider);
    }

    pub fn put_down(&mut self, player: &Player) {
        self.carried = false;
        // self.fall_speed += 1.0;
        // self.collider.set_y((self.y() + 20) as f32);
    }

    pub fn update(&mut self, player: &Player) {
        if self.carried {
            self.collider.set_x((player.collider.x()-self.offset.0) as f32);
            self.collider.set_y((player.collider.y()-self.offset.1) as f32);
        }
        else if self.in_air {
            self.fall_speed += 1.0;
            let predict = RectCollider::new(self.x() as f32, self.y() as f32 + self.fall_speed(), self.collider.width(), self.collider.height());
            if ground_collision(predict) {
                self.collider.set_y((720-(3*64 as i32)/2) as f32);
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

pub fn ground_collision(future: RectCollider) -> bool {
    let ground = RectCollider::new(0.0, 656.0, 1280.0, 64.0);
    if (future.is_touching(&ground)) {
        return true;
    }
    else {
        return false;
    }
}