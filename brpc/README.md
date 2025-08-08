# Rust Brpc Dirs Discription

1.  proto: The proto file for rpc, a simple echo.

1.  msg: bridge these cxx msg objects to rust.
    Thery are `not zero copy` and  opaque to Rust because the cxx types are not FFI.
    However, we can make large buffer, string, or bytes zero copy.
    Another zero copy way is to patch the brpc and provide a protoc where the
    serialization and deserialization are from binary to rust prost structs.
    See https://github.com/mesalock-linux/brpc-rs

1.  server: The cpp brpc server initialized by rust through cxx bridge,
            then the brpc service impl calls `service` by its cxx api.

1.  client: The cpp brpc client initialezed and used by rust through cxx bridge.
    Because the client shares the same cxx msg with `service`, they should be in
    the same cxx bridge. The rust rpc `service` implementation.

1.  test: A simple test binary to test the rust brpc client and server.
