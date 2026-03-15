use crate::user_request::UserRequest;
mod user_request;

fn main() {
    UserRequest::new().visit(|name| {});
}
