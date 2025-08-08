#[cfg(test)]
mod tests {
    use std::{
        sync::Arc,
        thread,
        time::Duration,
    };

    use client::*;
    use server::*;

    #[test]
    fn test_brpc() {
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

        let server_addr = format!("127.0.0.1:{}", port);

        println!("Server is running on {}", server_addr);

        let mut req = new_echo_request();
        echo_req_set_msg(req.pin_mut(), "Hello, BRPC!".to_string());

        let params = ConnParams {
            server_addr,
            protocol: "baidu_std".to_string(),
        };

        let mut client = new_echo_client();
        let result = client.pin_mut().init(&params);
        assert!(result.is_ok(), "Client failed to connect to server");

        let mut resp = new_echo_response();
        let result = client.echo(&req, resp.pin_mut());
        assert!(result.is_ok(), "Client failed to send echo request");
        assert!(
            resp.message().to_str().is_ok(),
            "Response message is not valid UTF-8"
        );
        println!("Echo response: {:?}", resp.message().to_str().unwrap());
    }
}
