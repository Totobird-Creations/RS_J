use javawithrust::prelude::*;


#[jclass(net.totobirdcreations.robot.rust.J2RS)]
pub struct J2RS;


#[jfuncs]
impl J2RS {
    fn hello() /*Result<(), String>*/ {
        println!("hello");
        crate::java::rs2j::RS2J::world()?;
        return Ok(());
    }
    fn sum(a : i32, b : i32) -> /*Result<*/i32/*, String>*/ {
        return Ok(a + b);
    }
}
