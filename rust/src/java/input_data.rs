use javawithrust::prelude::*;


#[jclass(net.totobirdcreations.robot.test.InputData)]
pub struct InputData {
    move_frontback : f32,
    move_rightleft : f32,
    turn_rightleft : f32
}
