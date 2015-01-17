extern crate wire;

use wire::SizeLimit;

fn main() {
    // Only allow incomming messages of at max 16 bytes, and verify that all of
    // our outgoing messages aren't over 8 bytes.
    let (read_limit, write_limit) = (SizeLimit::UpperBound(16),
                                     SizeLimit::UpperBound(8));

    // Connect to our running fib-server.
    // incoming: (u64, u64)
    // outgoing: u64
    let (i, mut o) = wire::connect_tcp("localhost", 8080, read_limit, write_limit).unwrap();

    // Send all the numbers from 0 to 10.
    for x in 0 .. 10 {
        o.send(&x).ok();
    }

    // Close our outgoing pipe. This is necessary because otherwise,
    // the iterator in the next segment will block forever awaiting responses
    // that will never be triggered.
    o.close();

    // Print everything that we get back.
    for a in i.into_blocking_iter() {
        let (x, fx): (u64, u64) = a;
        println!("{} -> {}", x, fx);
    }
}
