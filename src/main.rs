use crate::{
    new_mod::new_mod, new_proj::new_proj,
    user_request::UserRequest,
};
mod new_mod;
mod new_proj;
mod user_request;

fn main() {
    UserRequest::new().visit(
        |name| new_proj(name),
        |name| new_mod(name),
    );
}
