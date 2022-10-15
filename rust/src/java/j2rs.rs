use j4rs::{
    prelude::*,
    InvocationArg
};
use j4rs_derive::*;


#[call_from_java("net.totobirdcreations.robot.Rust.hello")]
fn hello() {
    println!("hello");
}


#[call_from_java("net.totobirdcreations.robot.Rust.rustSum")]
fn sum(a_instance : Instance, b_instance : Instance) -> Result<Instance, String> {
    let jvm = Jvm::attach_thread().unwrap();
    let a   = from_java!(jvm, <i32>::(a_instance));
    let b   = from_java!(jvm, <i32>::(b_instance));
    return into_java!(a + b);
}





macro_rules! from_java {
    ($jvm:expr, <$to:ty>::($from:expr)) => {
        $jvm.to_rust::<$to>($from).unwrap()
    };
}
pub(crate) use from_java;
macro_rules! into_java {
    ($from:expr) => {
        Instance::try_from(
            InvocationArg::try_from($from)
                .map_err(|error| format!("{}", error))
                .unwrap()
        )
            .map_err(|error| format!("{}", error))
    };
}
pub(crate) use into_java;
