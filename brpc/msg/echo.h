#pragma once

#include "echo.pb.h"
#include "rust/cxx.h"

namespace echo {
using EchoRequest = echo::v1::EchoRequest;
using EchoResponse = echo::v1::EchoResponse;

using namespace std;

unique_ptr<EchoRequest> new_echo_request();
unique_ptr<EchoResponse> new_echo_response();

void echo_req_set_msg(EchoRequest &req, const rust::String msg);
void echo_resp_set_msg(EchoResponse &resp, const rust::String msg);

}  // namespace echo
