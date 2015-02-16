# Wire
### An abstraction over TCP and Serialization

_"put a struct in one side and it comes out the other end"_

Wire is a library that makes writing applications that communicate via TCP easy.
If you've ever wanted to conceptually put a struct into one end of a tcp stream
and have it come out the other side, then Wire might be what you are looking for!

[Api docs](http://tyoverby.com/wire/wire/index.html)

## Example
Let's write a simple server that computes fibonacci numbers as a service.

These files can be found in the `examples` directory.
### Server

```rust
extern crate wire;

use std::thread::Thread;
use wire::SizeLimit;

fn fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    // Make a listener on 0.0.0.0:8080
    let (listener, _) = wire::listen_tcp(("0.0.0.0", 8080)).unwrap();

    // Only allow incoming messages of at max 8 bytes, and verify that we aren't
    // writing anything over 16 bytes.
    let (read_limit, write_limit) = (SizeLimit::Bounded(8),
                                     SizeLimit::Bounded(16));

    // Turn the listener into an iterator of connections.
    for connection in listener.into_blocking_iter() {
        // Spawn a new thread for each connection that we get.
        Thread::spawn(move || {
            // Upgrade the connection to read `u64` and write `(u64, u64)`.
            let (i, mut o) = wire::upgrade_tcp(connection, read_limit, write_limit);
            // For each `u64` that we read from the network...
            for x in i.into_blocking_iter() {
                // Send that number back with the computed value.
                o.send(&(x, fib(x))).ok();
            }
        });
    }
}

```

### Client

```rust
extern crate wire;

use wire::SizeLimit;

fn main() {
    // Only allow incomming messages of at max 16 bytes, and verify that all of
    // our outgoing messages aren't over 8 bytes.
    let (read_limit, write_limit) = (SizeLimit::Bounded(16),
                                     SizeLimit::Bounded(8));

    // Connect to our running fib-server.
    // incoming: (u64, u64)
    // outgoing: u64
    let (i, mut o) = wire::connect_tcp(("localhost", 8080), read_limit, write_limit).unwrap();

    // Send all the numbers from 0 to 10.
    for x in 0u64 .. 10u64 {
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

```

