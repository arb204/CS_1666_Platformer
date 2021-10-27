pub mod rect_collider {
    use crate::physics_controller::physics_controller::PhysicsController;

    pub struct RectCollider {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        can_move: bool
    }

    impl RectCollider {
        pub fn new(_x: f32, _y:f32, _width:f32, _height: f32, _can_move:bool)
            -> RectCollider
        {
            RectCollider {
                x: _x,
                y: _y,
                width: _width,
                height: _height,
                can_move: _can_move
            }
        }

        // getters
        pub fn x(&self) -> f32 { self.x }
        pub fn y(&self) -> f32 { self.y }
        pub fn width(&self) -> f32 { self.width }
        pub fn height(&self) -> f32 { self.height }
        pub fn can_move(&self) -> bool { self.can_move }

        // setters
        pub fn set_x(&mut self, _x: f32) { self.x = _x; }
        pub fn set_y(&mut self, _y: f32) { self.y = _y; }
        pub fn set_width(&mut self, _width: f32) { self.width = _width; }
        pub fn set_height(&mut self, _height: f32) { self.height = _height; }
        pub fn set_can_move(&mut self, _can_move: bool) { self.can_move = _can_move; }

        // debug: prints out a list of the rect colliders current state
        pub fn debug(&mut self) {
            println!("Rect Collider status:");
            println!("\tx: {}", self.x);
            println!("\ty: {}", self.y);
            println!("\twidth: {}", self.width);
            println!("\theight: {}", self.height);
            println!("\tcan_move: {}", self.can_move);
        }

        pub fn is_touching(&self, other: &RectCollider)
            -> bool
        {
            (self.height + self.y > other.y()) && (self.y < other.y() + other.height()) && (self.x + self.width > other.x()) && (self.x < other.x() + other.width())
        }

        // updates the rect collider every frame based on the position of the object
        pub fn update(&mut self, physics_controller: &PhysicsController) {
            self.x = physics_controller.x();
            self.y = physics_controller.y();
            self.can_move = physics_controller.can_move();
        }
    }
}
