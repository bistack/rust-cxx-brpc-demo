#[cxx::bridge]
mod ffi {

    extern "Rust" {
        type HelloMsg;

        fn new_hello(msg_num: i32) -> Box<HelloMsg>;
        fn add_msg_num(&mut self) -> i32;
    }
}

fn new_hello(msg_num: i32) -> Box<HelloMsg> {
    Box::new(HelloMsg::new(msg_num))
}

use hello_rust::HelloMsg as InnerHelloMsg;

pub struct HelloMsg {
    inner: InnerHelloMsg,
}

impl HelloMsg {
    pub fn new(msg_num: i32) -> HelloMsg {
        HelloMsg {
            inner: InnerHelloMsg::new(msg_num),
        }
    }

    pub fn add_msg_num(&mut self) -> i32 {
        self.inner.add_msg_num()
    }
}
