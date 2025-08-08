pub struct HelloMsg {
    msg_num: i32,
}

impl HelloMsg {
    pub fn new(msg_num: i32) -> HelloMsg {
        HelloMsg {
            msg_num,
        }
    }

    pub fn add_msg_num(&mut self) -> i32 {
        self.msg_num += 1;
        println!("HelloMsg::add_msg_num: {}", self.msg_num);
        self.msg_num
    }
}
