#[path = "simple.rs"]
mod simple;

#[path = "message_passing.rs"]
mod message_passing;

fn main() {
    simple::simple_thread();

    message_passing::message_passing();

}