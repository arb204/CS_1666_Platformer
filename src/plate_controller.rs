use crate::rect_collider::RectCollider;

#[derive(Copy, Clone)]
pub struct PlateController {
    plate_collider: RectCollider,
    plate_pressed: bool,
    gate_x: i32,
    gate_y: i32,
    gate_length: i32,
    gate_vertical: bool
}

impl PlateController {
    pub fn new(_plate_x: i32, _plate_y: i32, _gate_x: i32, _gate_y: i32, _gate_length: i32, _gate_vertical: bool)
        -> PlateController
    {
        PlateController {
            plate_collider: RectCollider::new(_plate_x as f32, (_plate_y+32) as f32, 64.0, 32.0),
            plate_pressed: false,
            gate_x: _gate_x,
            gate_y: _gate_y,
            gate_length: _gate_length,
            gate_vertical: _gate_vertical
        }
    }
    pub fn plate_collider(&self) -> RectCollider { self.plate_collider }
    pub fn plate_pressed(&self) -> bool { self.plate_pressed }
    pub fn gate_x(&self) -> i32 { self.gate_x }
    pub fn gate_y(&self) -> i32 { self.gate_y }
    pub fn gate_length(&self) -> i32 { self.gate_length }
    pub fn gate_vertical(&self) -> bool { self.gate_vertical }

    // update_plate: updates the pressure plate if the box collider is touching the plate
    pub fn update_plate(&mut self, box_collider: RectCollider) {
        self.plate_pressed = self.plate_collider.is_touching(&box_collider);
    }

    pub fn active_gate_collider(&self) -> RectCollider {
        if !self.plate_pressed {
            if self.gate_vertical {
                return RectCollider::new((self.gate_x+6) as f32, self.gate_y as f32, 44.0, self.gate_length as f32);
            } else {
                return RectCollider::new(self.gate_x as f32, (self.gate_y+6) as f32, self.gate_length as f32, 44.0);
            }
        }
        // the gate is off, no active collider
        RectCollider::new(0.0, 0.0, 0.0, 0.0)
    }
}