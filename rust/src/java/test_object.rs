use javawithrust::prelude::*;


#[jclass(net.totobirdcreations.robot.rust.TestObject)]
pub struct TestObject {
    content : String
}


#[jfuncs]
impl TestObject {
    fn create(text : String) -> /*Result<*/TestObject/*, String>*/ {
        return Ok(TestObject {
            content : text
        });
    }
}
