# Basic DNS Server in Rust

This is a basic DNS server implementation in Rust. This is a small learning and research project.

## How to run

1. Ensure you have `cargo (1.70)` installed locally
2. In the terminal run `./your_server.sh` to run your DNS server
3. In another terminal run `dig @127.0.0.1 -p 2053 +noedns example.com` to query the server

## How to test

Run `cargo test` to run the tests

## Resources

- [DNS RFC](https://datatracker.ietf.org/doc/html/rfc1035#section-4.1)
- [DNS Packet Format](https://www2.cs.duke.edu/courses/fall16/compsci356/DNS/DNS-primer.pdf)
- [DNS guide](https://github.com/EmilHernvall/dnsguide/blob/b52da3b32b27c81e5c6729ac14fe01fef8b1b593/chapter1.md)
- [Wikipedia](https://en.wikipedia.org/wiki/Domain_Name_System#DNS_message_format)
