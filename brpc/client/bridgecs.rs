#[cxx::bridge(namespace = "echo")]
pub mod ffi {
    unsafe extern "C++" {
        include!("brpc/msg/echo.h");
        include!("brpc/client/client.h");

        type EchoRequest;
        type EchoResponse;

        fn new_echo_request() -> UniquePtr<EchoRequest>;
        fn message(self: &EchoRequest) -> &CxxString;
        fn echo_req_set_msg(req: Pin<&mut EchoRequest>, msg: String);

        fn new_echo_response() -> UniquePtr<EchoResponse>;
        fn message(self: &EchoResponse) -> &CxxString;
        fn echo_resp_set_msg(resp: Pin<&mut EchoResponse>, msg: String);

        type EchoClient;

        fn new_echo_client() -> UniquePtr<EchoClient>;
        fn init(self: Pin<&mut EchoClient>, params: &ConnParams) -> Result<()>;
        fn echo(
            self: &EchoClient,
            req: &EchoRequest,
            resp: Pin<&mut EchoResponse>,
        ) -> Result<()>;
    }

    struct ConnParams {
        server_addr: String,
        protocol: String,
    }

    extern "Rust" {
        fn echo(req: &EchoRequest, resp: Pin<&mut EchoResponse>) -> Result<()>;
    }
}

use std::pin::Pin;

use anyhow::{
    anyhow,
    Result,
};
pub use ffi::{
    echo_req_set_msg,
    echo_resp_set_msg,
    new_echo_client,
    new_echo_request,
    new_echo_response,
    ConnParams,
    EchoClient,
    EchoRequest,
    EchoResponse,
};

pub fn echo(req: &EchoRequest, resp: Pin<&mut EchoResponse>) -> Result<()> {
    let msg = req
        .message()
        .to_str()
        .map_err(|e| anyhow!("Failed to convert message to string: {}", e))?;
    let msg = msg.to_string() + " from Rust!";
    echo_resp_set_msg(resp, msg);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::ffi::*;

    #[test]
    fn msg_works() {
        let mut req = new_echo_request();
        echo_req_set_msg(req.pin_mut(), "Hello, World!".to_string());
        let msg = req.message();
        assert_eq!(msg.to_str().unwrap(), "Hello, World!");

        let msg = msg.to_str().unwrap().to_string() + " from Rust!";

        let mut resp = new_echo_response();
        echo_resp_set_msg(resp.pin_mut(), msg);
        assert_eq!(
            resp.message().to_str().unwrap(),
            "Hello, World! from Rust!"
        );
    }

    #[test]
    fn client_works() {
        let mut req = new_echo_request();
        echo_req_set_msg(req.pin_mut(), "Hello, World! from Rust".to_string());
        let msg = req.message();
        assert_eq!(msg.to_str().unwrap(), "Hello, World! from Rust");

        let params = ConnParams {
            server_addr: "127.0.0.1:8080".to_string(),
            protocol: "baidu_std".to_string(),
        };

        let mut client = new_echo_client();
        if client.pin_mut().init(&params).is_ok() {
            let mut resp = new_echo_response();
            let _ = client.echo(&req, resp.pin_mut());
        }
    }
}
