#include <iostream>

using namespace rust;

int main() {
    Box<HelloMsg> msg = new_hello(0);
    msg->add_msg_num();
    return 0;
}
