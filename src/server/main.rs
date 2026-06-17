use std::net::UdpSocket;

struct Server {
    address: String,
}

impl Server {
    fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
        }
    }

    fn send(&self) -> std::io::Result<()> {
        let socket = UdpSocket::bind(&self.address).expect("Could not create udp socket.");
        loop {
            let mut buf = [0; 10];
            let (size, src) = socket.recv_from(&mut buf)?;

            println!("Received {} bytes from {}: {:?}", size, src, &buf[..size]);
        }
    }
}

fn main() {
    let server = Server::new("127.0.0.1:34254");
    server.send().expect("Issue sending.");
}
