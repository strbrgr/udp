use std::net::UdpSocket;

struct Client {
    address: String,
}

impl Client {
    fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
        }
    }

    fn send(&self) -> std::io::Result<()> {
        let socket = UdpSocket::bind(&self.address).expect("Could not create udp socket.");
        loop {
            let msg = "msg";
            let sent = socket.send_to(msg.as_bytes(), "127.0.0.1:8080")?;
            println!("Sent {sent} bytes.");
        }
    }
}

fn main() {
    let client = Client::new("0.0.0.0:0"); // OS you pick
    client.send().expect("Issue sending.");
}
