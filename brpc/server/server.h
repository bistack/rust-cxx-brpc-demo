#pragma once

#include <memory>
#include <brpc/server.h>
#include "rust/cxx.h"

#include "echo.pb.h"

namespace echo {
using EchoService = echo::v1::EchoService;
using EchoRequest = echo::v1::EchoRequest;
using EchoResponse = echo::v1::EchoResponse;

using namespace std;

struct ServerParams;
class EchoServiceImpl : public EchoService {
 public:
    EchoServiceImpl() = default;
    virtual ~EchoServiceImpl() = default;
    virtual void DoEcho(google::protobuf::RpcController* cntl_base,
                        const EchoRequest* request, EchoResponse* response,
                        google::protobuf::Closure* done);
};

class EchoServer {
 public:
    EchoServer() = default;
    ~EchoServer() = default;

    void init();
    void start(const ServerParams& params) const;
    int get_port() const;

 private:
    unique_ptr<brpc::Server> server_;
    unique_ptr<EchoServiceImpl> service_;
};

unique_ptr<EchoServer> new_echo_server();

}  // namespace echo
