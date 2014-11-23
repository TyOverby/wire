extern crate wire;

fn main() {
    let (i, o) = wire::connect::<(u64, u64), u64>("localhost", 8080).unwrap();
    for x in range(0, 10) {
        o.send(x);
    }
    for _ in range(0u, 10) {
        println!("{}", i.recv());
    }
}
