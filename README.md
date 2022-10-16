# ipp

This program listens for HTTP requests on all interfaces and responds with the public facing IP address of the request in raw text format. It selects the first valid and globally routable IP in the `X-Forwarded-For` header then falls back to the remote IP of the connection.

## Public deployment

- http://ip4.raftar.io
- http://ip6.raftar.io

## Custom deployment

A docker image is available at [`ghcr.io/raftario/ipp`](https://github.com/raftario/ipp/pkgs/container/ipp). Standalone binaries are also uploaded as GitHub Actions artifacts for popular OSes and architectures. If neither fit your usecase binaries can be built from source by running `cargo build --release`.

By default the service listens on port 8080 but this can be changed by either passing a custom port as command line argument or setting the `PORT` environment variable.
