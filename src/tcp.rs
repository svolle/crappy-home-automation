use std::io::prelude::*;
use std::net::{Shutdown, SocketAddr, TcpListener};
use std::thread;
use std::thread::sleep;
use std::time;

use rand::prelude::*;

fn generate_reading(counter: u32) -> String {
    let mut rng = thread_rng();
    let dist = rand::distributions::Normal::new(22.0, 3.0);
    let mut t = rng.sample(dist);

    if counter % 2 != 0 {
        t = -t;
    }

    format!("{:.*}°C\n", 1, t)
}

pub fn simple_tcp_server() {
    let addr: SocketAddr = ([127, 0, 0, 1], 40000).into();
    let listener = TcpListener::bind(addr).unwrap();

    println!("TCP server listening on {}", addr);

    thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut counter = 0;

            loop {
                match stream.write(generate_reading(counter).as_bytes()) {
                    Err(e) => {
                        eprintln!("{}", e);
                        break;
                    }
                    _ => (),
                };
                counter += 1;
                sleep(time::Duration::from_secs(5));

                let mut rng = thread_rng();
                if rng.gen_range(1, 4) > 2 {
                    println!("Dropping connection!");
                    stream.shutdown(Shutdown::Both).unwrap();
                    break;
                };
            }
        }
    });
}
