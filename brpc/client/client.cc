#include <memory>
#include <string>

#include "client.h"
#include "brpc/client/bridgecs.rs.h"

namespace echo {

unique_ptr<EchoClient> new_echo_client() { return make_unique<EchoClient>(); }

void EchoClient::init(const ConnParams &params) {
    brpc::ChannelOptions options;
    const char *server_addr = params.server_addr.data();
    options.protocol = params.protocol.data();

    channel_ = make_unique<brpc::Channel>();
    if (channel_ == nullptr) {
        string err_msg = "Fail to create channel";
        throw runtime_error(err_msg);
    }

    if (channel_->Init(server_addr, &options) != 0) {
        string err_msg = "Fail to initialize channel";
        throw runtime_error(err_msg);
    }

    stub_ = make_unique<EchoService_Stub>(channel_.get());
}

void EchoClient::echo(const EchoRequest &req, EchoResponse &resp) const {
    static int log_id = 0;

    brpc::Controller cntl;
    cntl.set_log_id(log_id++);

    stub_->DoEcho(&cntl, &req, &resp, nullptr);

    if (cntl.Failed()) {
        string err_msg = "RPC failed: " + cntl.ErrorText();
        throw runtime_error(err_msg);
    }
}
}  // namespace echo
