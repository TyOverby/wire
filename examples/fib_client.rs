extern crate wire;

fn main() {
    let (i, o) = wire::connect::<(u64, u64), u64>("localhost", 8080).unwrap();

    o.send_all(range(0, 10));

    for x in i.into_blocking_iter().take(10) {
        println!("{}", x);
    }
}
