# Wire
### An abstraction over TCP and Serialization

Wire is a library that makes writing applications that communicate via TCP easy.
If you've ever wanted to conceptually put a struct into one end of a tcp stream
and have it come out the other side, then Wire might be what you are looking for!

## Example
Let's write a simple server that computes fibonacci numbers as a service.

These files can be found in the `examples` directory.
### Server

```rust
extern crate wire;

use std::task::spawn;

fn fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    // Make a listener on 0.0.0.0:8080
    let (listener, _) = wire::listen("0.0.0.0", 8080).unwrap();
    // Turn the listener into an iterator of connections.
    for connection in listener.into_blocking_iter() {
        // Spawn a new thread for each connection that we get.
        spawn(proc() {
            // Upgrade the connection to read `u64` and write `(u64, u64)`.
            let (i, mut o) = wire::upgrade(connection);
            // For each `u64` that we read from the network...
            for x in i.into_blocking_iter() {
                // Send that number back with the computed value.
                o.send(&(x, fib(x)));
            }
        });
    }
}

```

### Client

```rust
extern crate wire;

fn main() {
    // Connect to our running fib-server.
    // incoming: (u64, u64)
    // outgoing: u64
    let (i, mut o) = wire::connect("localhost", 8080).unwrap();

    // Send all the numbers from 0 to 10.
    for x in range(0u64, 10u64) {
        o.send(&x);
    }
    // Close our outgoing pipe.
    o.close();

    // Print everything that we get back.
    for a in i.into_blocking_iter() {
        let (x, fx): (u64, u64) = a;
        println!("{} -> {}", x, fx);
    }
}

```

