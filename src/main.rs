mod http;
mod tcp;
mod udp;

fn main() {
    udp::simple_udp_server();
    tcp::simple_tcp_server();
    http::simple_http_server(); // blocking
}
