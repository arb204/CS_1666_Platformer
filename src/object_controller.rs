pub mod object_controller {
    //ObjectController: controls physics and collision for carryable objects (UNFINISHED)
    // use crate::physics_controller::physics_controller::PhysicsController;
    use crate::rect_collider::rect_collider::RectCollider;
    pub struct ObjectController {
        collider: RectCollider,
        carried: bool,
    }

    impl ObjectController {
        pub fn new(_collider: RectCollider) -> ObjectController
        {
            ObjectController {
                collider: _collider,
                carried: false
            }
        }

        pub fn x(&self) -> i32 { self.collider.x() as i32}
        pub fn y(&self) -> i32 { self.collider.y() as i32}

        pub fn picked_up(&mut self) {
            self.carried = true;
            // pin or disable rect collider

        }

        pub fn put_down(&mut self) {
            self.carried = false;
            // enable rect collider

        }
    }
}