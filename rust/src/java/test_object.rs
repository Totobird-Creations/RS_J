use javawithrust::prelude::*;


#[jclass(net.totobirdcreations.robot.rust.TestObject)]
pub struct TestObject {
    content : String
}


#[jfuncs]
impl TestObject {
    fn concat(this : TestObject, data : String) /*Result<(), String>*/ {
        println!("{}", this.content + &data);
        return Ok(());
    }
}
