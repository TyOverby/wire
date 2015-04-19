extern crate wire;
use std::io::BufRead;
use wire::udp;

fn get_sockets() -> ((&'static str, u16), Option<(&'static str, u16)>) {
    let mut args = std::env::args();
    args.next();
    let p1 = args.next()
                 .and_then(|s| s.parse().ok())
                 .expect("expected port number");
    let a2 = args.next()
                 .and_then(|s| s.parse().ok())
                 .map(|port| ("127.0.0.1", port));
    (("127.0.0.1", p1), a2)
}

fn main() {
    let (mine, theirs) = get_sockets();
    let (sender, receiver) = udp::bind(mine).unwrap();

    if let Some(theirs) = theirs {
        let stdin = std::io::stdin();
        for line in stdin.lock().lines().filter_map(|a| a.ok()) {
            sender.send(&line, theirs);
        }
    } else {
        for (from, m) in receiver.into_blocking_iter() {
            let m: String = m;
            println!("{}: {}", from, m);
        }
    }
}
