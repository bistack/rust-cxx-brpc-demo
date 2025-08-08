#include <memory>
#include <string>

#include "echo.h"

namespace echo {

unique_ptr<EchoRequest> new_echo_request() {
    return make_unique<EchoRequest>();
}

unique_ptr<EchoResponse> new_echo_response() {
    return make_unique<EchoResponse>();
}

void echo_req_set_msg(EchoRequest &req, const rust::String msg) {
    req.set_message(string(msg));
}

void echo_resp_set_msg(EchoResponse &resp, const rust::String msg) {
    resp.set_message(string(msg));
}

}  // namespace echo
