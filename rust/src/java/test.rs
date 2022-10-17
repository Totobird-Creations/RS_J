use javawithrust::prelude::*;


#[jclass(net.totobirdcreations.robot.test.Test)]
pub struct Test;


#[jfuncs]
impl Test {

    fn hello() /*Result<(), String>*/ {
        println!("Hello");
        Test::world()?;
        return Ok(());
    }

    fn world();

    fn add(a : i32, b : i32) -> /*Result<*/i32/*, String>*/ {
        return Ok(a + b);
    }

}
