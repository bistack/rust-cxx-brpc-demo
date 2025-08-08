use bridge_rs::ffi::new_hello;

fn main() {
    let mut hello = new_hello(42);
    let hello = hello.pin_mut();
    hello.add_msg_num();
}
