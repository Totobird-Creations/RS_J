use std::convert::TryFrom;

use j4rs::{
    prelude::*,
    InvocationArg
};
use j4rs_derive::*;


#[call_from_java("net.totobirdcreations.robot.Link.hello")]
fn hello() {
    println!("hello");
}


#[call_from_java("net.totobirdcreations.robot.Link.rustSum")]
fn sum(a_instance : Instance, b_instance : Instance) -> Result<Instance, String> {
    let jvm = Jvm::attach_thread().unwrap();
    let a   = jvm.to_rust::<i32>(a_instance).unwrap();
    let b   = jvm.to_rust::<i32>(b_instance).unwrap();
    return into_java(a + b);
}


macro_rules! from_java {
    ($from:expr) => {
        // TODO.
    };
}

fn into_java<T>(from : T) -> Result<Instance, String>
    where InvocationArg : TryFrom<T>
{
    return Instance::try_from(
        InvocationArg::try_from(from)
            .map_err(|_| format!("InvocationArg conversion failed.") /* TODO : PRINT ACTUAL ERROR*/)
            .unwrap()
    )
        .map_err(|error| format!("{}", error));
}
