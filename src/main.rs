extern crate config;
extern crate serde;

fn main() {
    if let Err(e) = user_service::run() {
        panic!(e);
    }
}
