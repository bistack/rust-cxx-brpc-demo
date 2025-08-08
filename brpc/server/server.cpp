#include "server.h"
#include "brpc/client/bridgecs.rs.h"
#include "brpc/server/bridgeserver.rs.h"

namespace echo {

void EchoServiceImpl::DoEcho(google::protobuf::RpcController* cntl_base,
                           const EchoRequest* request, EchoResponse* response,
                           google::protobuf::Closure* done) {
    // This object helps you to call done->Run() in RAII style. If you need
    // to process the request asynchronously, pass done_guard.release().
    brpc::ClosureGuard done_guard(done);

    // Call a Rust echo method here.
    echo(*request, *response);
}

int EchoServer::get_port() const {
    if (server_) {
        const butil::EndPoint& addr = server_->listen_address();
        return addr.port;
    }
    return 0;
}

void EchoServer::init() {
    if (server_) {
        throw runtime_error("Server already initialized");
    }

    server_ = make_unique<brpc::Server>();
    service_ = make_unique<EchoServiceImpl>();

    // Add the service into server. Notice the second parameter, because the
    // service is put on stack, we don't want server to delete it, otherwise
    // use brpc::SERVER_OWNS_SERVICE.
    int ret =
        server_->AddService(service_.get(), brpc::SERVER_DOESNT_OWN_SERVICE);
    if (ret != 0) {
        string err_msg = "Fail to add service, err code: " + to_string(ret);
        throw runtime_error(err_msg);
    }
}

unique_ptr<EchoServer> new_echo_server() {
    unique_ptr<EchoServer> serv = make_unique<EchoServer>();
    serv->init();

    return serv;
}

void EchoServer::start(const ServerParams& params) const {
    if (!server_ || !service_) {
        throw runtime_error("Server not initialized");
    }

    int port = params.port;
    butil::EndPoint point = butil::EndPoint(butil::IP_ANY, port);

    brpc::ServerOptions options;
    options.idle_timeout_sec = params.idle_timeout_secs;

    int ret = server_->Start(point, &options);
    if (ret != 0) {
        string err_msg = "Fail to start server, err code: " + to_string(ret);
        throw runtime_error(err_msg);
    }
    // Wait until Ctrl-C is pressed, then Stop() and Join() the server.
    server_->RunUntilAskedToQuit();
}

}  // namespace echo
