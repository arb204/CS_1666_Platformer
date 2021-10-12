pub mod animation_controller {
    use eval::eval;
    use crate::player::player::Player;
    use crate::physics_controller::physics_controller::PhysicsController;
    pub struct AnimController {
        id: String,
        rows: i32,
        columns: i32,
        width: i32,
        height: i32,
        animations: Vec<Anim>
        parent: &Player,
    }

    impl AnimController {
        pub fn new(_id:String, _rows: i32, _columns: i32, _width:i32, _height: i32, _animations: Vec<Anim>, _parent: &Player)
            -> AnimController
        {
            AnimController {
                id: _id,
                rows: _rows,
                columns: _columns,
                width: _width,
                height: _height,
                animations: _animations,
                parent: _parent
            }
        }
    }

    pub struct Anim {
        frames: Vec<i32>,
        condition: Condition
    }

    impl Anim {
        pub fn new(_frames: Vec<i32>, _cstring:String, _cprio: i32)
            -> Anim
        {
            Anim {
                frames: _frames,
                condition: Condition::new(_cstring, _cprio)
            }
        }

        pub fn new(_frames: Vec<i32>, _condition: Condition)
            -> Anim
        {
            Anim {
                frames: _frames,
                condition: _condition
            }
        }
    }

    struct Condition {
        condition: String,
        priority: i32
    }

    impl Condition {

        fn new(_condition: String, _priority: i32)
            -> Condition
        {
            Condition {
                condition: _condition,
                priority: _priority
            }
        }

        //getters
        fn condition(&self) -> String { self.condition }
        fn priority(&self) -> i32 { self.priority }

        fn is_met(&self) -> bool {
            eval(self.condition) == true
        } 
    }
}