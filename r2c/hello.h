#pragma once
#include <memory>

struct HelloMsg {
    int msg_num;

 public:
    explicit HelloMsg(int msg_num) : msg_num(msg_num) {}

    int add_msg_num();
};

std::unique_ptr<HelloMsg> new_hello(int msg_num);
