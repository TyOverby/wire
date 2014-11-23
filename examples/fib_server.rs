extern crate wire;

use std::task::spawn;

fn fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    for connection in wire::listen("0.0.0.0", 8080).unwrap().iter() {
        spawn(proc() {
            let (i, o) = wire::upgrade::<u64, (u64, u64)>(connection);
            for x in i.iter() {
                o.send((x, fib(x)));
            }
        });
    }
}
