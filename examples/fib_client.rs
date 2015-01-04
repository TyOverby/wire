extern crate wire;

fn main() {
    // Connect to our running fib-server.
    // incoming: (u64, u64)
    // outgoing: u64
    let (i, mut o) = wire::connect("localhost", 8080).unwrap();

    // Send all the numbers from 0 to 10.
    for x in range(0u64, 10u64) {
        o.send(&x).ok();
    }
    // Close our outgoing pipe.
    o.close();

    // Print everything that we get back.
    for a in i.into_blocking_iter() {
        let (x, fx): (u64, u64) = a;
        println!("{} -> {}", x, fx);
    }
}
