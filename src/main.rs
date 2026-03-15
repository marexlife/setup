use std::process::exit;

use crate::{
    new_mod::new_mod, new_proj::new_proj,
    user_request::UserRequest,
};
mod file;
mod new_mod;
mod new_proj;
mod user_request;
mod utils;

fn main() {
    UserRequest::new().visit(
        |name| {
            new_proj(name);
            exit(0);
        },
        |name| {
            new_mod(name);
            exit(0);
        },
    );
}
