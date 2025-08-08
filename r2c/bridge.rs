#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("r2c/hello.h");

        type HelloMsg;

        fn new_hello(msg_num: i32) -> UniquePtr<HelloMsg>;
        fn add_msg_num(self: Pin<&mut HelloMsg>) -> i32;
    }
}
