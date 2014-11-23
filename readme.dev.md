# Wire
### An abstraction over TCP and Serialization

Wire is a library that makes writing applications that communicate via TCP easy.
If you've ever wanted to conceptually put a struct into one end of a tcp stream
and have it come out the other side, then Wire might be what you are looking for!

## Example
Let's write a simple server that computes fibonacci numbers as a service.

These files can be found in the `examples` directory.
### Server

^code(./examples/fib_server.rs)

### Client

^code(./examples/fib_client.rs)

