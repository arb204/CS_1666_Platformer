pub mod rect_collider {
    pub struct RectCollider {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        rotation: f32,
        can_move: bool
    }

    impl RectCollider {
        fn new(_x: f32, _y:f32, _width:f32, _height: f32, _rotation: f32, _can_move:bool)
            -> RectCollider
        {
            RectCollider {
                x: _x,
                y: _y,
                width: _width, 
                height: _height,
                rotation: _rotation % 360.0,
                can_move: _can_move
            }
        }

        fn has_collided_with(&self, other: RectCollider)
            -> bool
        {
            self.x < other.x && self.x+self.width > other.x && self.y < other.y && self.y+self.height > other.y
        }
    }
}