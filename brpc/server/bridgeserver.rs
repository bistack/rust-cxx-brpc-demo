// The brpc server is thread-safe.
// So we use unsafe interior mutability with sync and send.

#[cxx::bridge(namespace = "echo")]
pub mod ffi {
    unsafe extern "C++" {
        include!("brpc/server/server.h");

        type EchoServer;

        fn new_echo_server() -> Result<UniquePtr<EchoServer>>;

        fn start(self: &EchoServer, params: &ServerParams) -> Result<()>;

        fn get_port(self: &EchoServer) -> i32;
    }

    struct ServerParams {
        port: i32,
        idle_timeout_secs: i32,
    }
}

unsafe impl Send for ffi::EchoServer {}
unsafe impl Sync for ffi::EchoServer {}

pub use ffi::{
    new_echo_server,
    EchoServer,
    ServerParams,
};

#[cfg(test)]
mod test {
    use std::{
        sync::Arc,
        thread,
        time::Duration,
    };

    use super::ffi::*;

    #[test]
    fn test_start_brpc_server() {
        let server = new_echo_server().unwrap();
        let server = Arc::new(server);
        let server_clone = Arc::clone(&server);

        let params = ServerParams {
            port: 0,
            idle_timeout_secs: 2,
        };

        let _guard = thread::spawn(move || {
            let result = server_clone.start(&params);
            assert!(
                result.is_ok(),
                "Failed to start BRPC server on port {}",
                &params.port
            );
        });

        // Wait for the server to start
        thread::sleep(Duration::from_secs(2));
        let port = server.get_port();
        assert!(port > 0, "Server port should be greater than 0");
    }
}
