use javawithrust::prelude::*;


#[jclass(net.totobirdcreations.robot.rust.TestObject)]
pub struct TestObject {
    content : String
}
impl TryFrom<TestObject> for InvocationArg {
    type Error = errors::J4RsError;
    fn try_from(arg : TestObject) -> errors::Result<InvocationArg> {
        return Ok(InvocationArg::new(&arg, __jwrs_classname_TestObject!()));
    }
}


#[jfuncs]
impl TestObject {
    fn create(text : String) -> /*Result<*/TestObject/*, String>*/ {
        return Ok(TestObject {
            content : text
        });
    }
    fn concat(mut this : TestObject, text : String) -> /*Result<*/TestObject/*, String>*/ {
        this.content += &text;
        return Ok(this);
    }
}
