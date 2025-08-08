#pragma once

#include <memory>

#include <brpc/channel.h>

#include "echo.pb.h"

namespace echo {
using EchoRequest = echo::v1::EchoRequest;
using EchoResponse = echo::v1::EchoResponse;
using EchoService_Stub = echo::v1::EchoService_Stub;

using namespace std;

struct ConnParams;

class EchoClient final {
 public:
    EchoClient() = default;
    ~EchoClient() = default;

    void init(const ConnParams &params);
    void echo(const EchoRequest &req, EchoResponse &resp) const;

 private:
    unique_ptr<EchoService_Stub> stub_;
    // The channel is used later in the echo() method.
    unique_ptr<brpc::Channel> channel_;
};

unique_ptr<EchoClient> new_echo_client();

}  // namespace echo
