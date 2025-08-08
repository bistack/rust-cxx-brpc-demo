# CXX examples

## c2r

cpp call rust.

## r2c

rust call cpp.

## rcr

rust call cpp and cpp call rust.

See `@com_github_dtolnay_cxx//demo`.

## brpc

rust use brpc that is a c++ rpc framework.

The server has 3 layers, rust_cpp_rust, those are rust start cpp brpc server,
the cpp brpc implementation calls the real rust implementation by passing
request to rust and getting response back.

The client has 2 laysers, rust_cpp, those are rust initialize cpp brpc client,
the rust code calls the cpp brpc methods by passing the rust request to cpp,
and getting the rust response back.
