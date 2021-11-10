pub mod object_controller {
    //ObjectController: controls physics and collision for carryable objects (UNFINISHED)
    use crate::rect_collider::rect_collider::RectCollider;
    use crate::player::player::Player;
    pub struct ObjectController {
        collider: RectCollider,
        carried: bool,
        pub offset: (i32, i32),
    }

    impl ObjectController {
        pub fn new(_collider: RectCollider) -> ObjectController
        {
            ObjectController {
                collider: _collider,
                carried: false,
                offset: (0, 0)
            }
        }

        pub fn x(&self) -> i32 { self.collider.x() as i32}
        pub fn y(&self) -> i32 { self.collider.y() as i32}
        pub fn carried(&self) -> bool { self.carried }
        pub fn collider(&self) -> RectCollider { self.collider }

        pub fn picked_up(&mut self, player: &Player) {
            self.carried = true;
            // pin or disable rect collider
            self.collider.set_y((self.y() - 20) as f32);
            self.offset = get_offset(self.collider(), player.collider);
        }

        pub fn put_down(&mut self, player: &Player) {
            self.carried = false;
            // enable rect collider
            self.collider.set_y((self.y() + 20) as f32);    // change later to gravity pull

        }

        pub fn update(&mut self) {
            if self.carried {
                self.collider.set_x((self.x()+self.offset.0) as f32);
                self.collider.set_y((self.y()+self.offset.1) as f32);
            }
        }
    }

    pub fn get_offset(inner: RectCollider, outer: RectCollider) -> (i32, i32) {
        let y = outer.y() - inner.y();
        let x = outer.x() - inner.x();
        (x as i32, y as i32)
    }
}