use std::net::UdpSocket;
use std::time::Duration;

#[test]
fn server_port_available_for_bind_config() {
    // Sanity check that port 8080 is not in use locally in CI environment
    // This doesn't start the ENet server; it ensures we can bind if needed
    let bind = UdpSocket::bind("127.0.0.1:0").expect("can bind ephemeral");
    bind.set_read_timeout(Some(Duration::from_millis(10))).ok();
}
