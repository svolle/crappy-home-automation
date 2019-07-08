use rand::prelude::*;
use rand::Rng;
use std::mem::transmute;
use std::net::{SocketAddr, UdpSocket};
use std::str;
use std::thread;

static UNKNOWN_CMD_MSG: &str = "Unknown command\n";
static GET_CMD: [u8; 3] = *b"GET";

fn generate_reading() -> String {
    let mut rng = thread_rng();

    if rng.gen_range(1, 4) > 2 {
        let gibberish: u64 = rng.gen_range(1000000, 1000000000000);
        let gibberish: [u8; 8] = unsafe { transmute(gibberish.to_be()) };

        println!("Sending gibberish");
        return String::from(unsafe { str::from_utf8_unchecked(&gibberish) });
    }

    let dist = rand::distributions::Normal::new(20.0, 2.0);
    let t = rng.sample(dist);

    format!("{:.*}°C\n", 1, t)
}

pub fn simple_udp_server() {
    let mut buf = [0; 3];
    let addr: SocketAddr = ([127, 0, 0, 1], 20000).into();
    let socket = UdpSocket::bind(&addr).unwrap();

    println!("UDP server listening on {}", &addr);

    thread::spawn(move || loop {
        let (_, address) = socket.recv_from(&mut buf).unwrap();

        let response = if buf.starts_with(&GET_CMD) {
            println!("Got GET command");
            generate_reading()
        } else {
            println!("Got unknown command '{}'", String::from_utf8_lossy(&buf));
            String::from(UNKNOWN_CMD_MSG)
        };

        socket.send_to(response.as_bytes(), address).unwrap();
    });
}
