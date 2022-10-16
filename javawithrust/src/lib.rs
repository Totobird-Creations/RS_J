pub mod prelude {
    pub use super::jclass;
    pub use super::jfuncs;
    pub use j4rs::{
        errors,
        prelude::*,
        InvocationArg
    };
    pub use j4rs_derive::call_from_java;
    pub use paste;
    pub use serde;
}

pub use javawithrust_macro::*;
