mod r#priv {
    include!(concat!(env!("OUT_DIR"), "/terraform.rs"));
}

pub use r#priv::kubernetes::*;
