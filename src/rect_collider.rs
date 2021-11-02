pub mod rect_collider {
    use crate::physics_controller::physics_controller::PhysicsController;
    use sdl2::rect::Rect;
    use std::cmp::{min,max};

    #[derive(Copy, Clone)]
    pub struct RectCollider {
        x: f32,
        y: f32,
        width: f32,
        height: f32
    }

    impl RectCollider {
        pub fn new(_x: f32, _y:f32, _width:f32, _height: f32)
            -> RectCollider
        {
            RectCollider {
                x: _x,
                y: _y,
                width: _width,
                height: _height
            }
        }

        // getters
        pub fn x(&self) -> f32 { self.x }
        pub fn y(&self) -> f32 { self.y }
        pub fn width(&self) -> f32 { self.width }
        pub fn height(&self) -> f32 { self.height }

        // setters
        pub fn set_x(&mut self, _x: f32) {self.x = _x}
        pub fn set_y(&mut self, _y: f32) {self.y = _y}

        // debug: prints out a list of the rect colliders current state
        /*pub fn debug(&mut self) {
            println!("Rect Collider status:");
            println!("\tx: {}", self.x);
            println!("\ty: {}", self.y);
            println!("\twidth: {}", self.width);
            println!("\theight: {}", self.height);
        }*/

        pub fn is_touching(&self, other: &RectCollider)
            -> bool
        {
            (self.height + self.y > other.y()) && (self.y < other.y() + other.height()) && (self.x + self.width > other.x()) && (self.x < other.x() + other.width())
        }

        pub fn get_overlap(&self, other: &RectCollider)
            -> Rect
        {
            if !self.is_touching(other) {
                Rect::new(self.x as i32, self.y as i32, 0, 0)
            } else {
                Rect::new(max(self.x as i32, other.x() as i32), max(self.y as i32, other.y() as i32), (max((self.x+self.width-other.x()) as i32, (other.x()+other.width()-self.x) as i32)) as u32, (max((self.y+self.height-other.y()) as i32, (other.y()+other.height()-self.y) as i32)) as u32)
            }
        }

        pub fn contains_point(&self, _x: f32, _y: f32)
            -> bool
        {
            (self.height + self.y > _y) && (self.y < _y) && (self.x + self.width > _x) && (self.x < _x)
        }

        // updates the rect collider every frame based on the position of the object
        pub fn update(&mut self, physics_controller: &PhysicsController) {
            self.x = physics_controller.x();
            self.y = physics_controller.y();
        }
    }
}
