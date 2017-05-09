extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate url;
extern crate yup_oauth2;

mod common;
pub mod cloudkms1;
pub mod datastore1;
pub mod oauth2 {
    pub use super::yup_oauth2::*;
}

pub use common::{Error, Hub, Result};
