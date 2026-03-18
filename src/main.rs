use crate::{
    new_mod::new_mod, new_proj::new_proj,
    run_proj::run_proj,
    user_request::UserRequest,
};
mod file;
mod new_mod;
mod new_proj;
mod run_proj;
mod shared;
mod user_request;
mod utils;

fn main() {
    UserRequest::new()
        .chose(new_proj, new_mod, run_proj);
}
