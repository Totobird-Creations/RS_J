#![crate_type = "dylib"]

use j4rs::InvocationArg;
use j4rs::prelude::*;
use j4rs_derive::*;
use serde::Deserialize;


#[call_from_java("net.totobirdcreations.robot.Link.rust_sum")]
pub extern fn sum(a_instance : Instance, b_instance : Instance) -> Result<Instance, String> {
    let jvm       = Jvm::attach_thread().unwrap();
    let a   : i32 = jvm.to_rust(a_instance).unwrap();
    let b   : i32 = jvm.to_rust(b_instance).unwrap();
    let sum       = a + b;
    let i         = InvocationArg::try_from(sum).unwrap();
    return Ok(Instance::try_from(i).unwrap());
}
