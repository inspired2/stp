# Description

this repo is for educational purposes only.
it contains homework projects for rust course, provided by otus.ru

## to test:

### 1. run server

cargo run --example smart_socket_server

server starts with 2 smart sockets present in SmartDeviceList, which dispatches commands received from the clients to appropriate devices.
SmartSocket names: "s1", "s2"
Available command codes will be printed upon running client.

### 2. run client (multiple clients are supported)

cargo run --example smart_socket_client

### 3. run commands from the list


# UPDATE

## Async server for SmartSocketUI:

RUST_LOG=debug cargo run --example async_server
