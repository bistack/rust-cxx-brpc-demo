#include <iostream>
#include "hello.h"

using std::make_unique;
using std::unique_ptr;

unique_ptr<HelloMsg> new_hello(int msg_num) {
    return make_unique<HelloMsg>(msg_num);
}

int HelloMsg::add_msg_num() {
    ++msg_num;
    std::cout << "HelloMsg::add_msg_num " << msg_num << std::endl;
    return msg_num;
}
