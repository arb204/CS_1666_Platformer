pub mod portal_controller {
    use crate::physics_controller::physics_controller::PhysicsController;
    use crate::sdl2;
    pub struct PortalController {
        wand_rotation: f32,
        portal0_x: f32,
        portal0_y: f32,
        portal1_x: f32,
        portal1_y: f32,
        should_rotate: bool,
        physics: PhysicsController
    }

    impl PortalController {
        pub fn new(_physics: PhysicsController)
            -> PortalController
        {
            PortalController {
                wand_rotation: 0.0,
                portal0_x: 0.0,
                portal0_y: 0.0,
                portal1_x: 0.0,
                portal1_y: 0.0,
                should_rotate: false,
                physics: _physics
            }
        }

        // make it so the wand doesn't rotate (like in a level complete)
        pub fn freeze(&mut self) { self.should_rotate = false; }

        // update the physics controllers so the wand can rotate properly
        pub fn update(&mut self, newphysics: PhysicsController) {
            self.physics = newphysics;
        }

        //next_rotation: returns a float indicating the angle of the next frame
        pub fn next_rotation(&mut self, mouse_x:i32, mouse_y: i32) -> f32 {
            if self.should_rotate {
                self.wand_rotation = ((mouse_y as f32-self.physics.y()).abs()/(mouse_x as f32-self.physics.x()).abs()).atan();
            }
            self.wand_rotation
        }
    }
}