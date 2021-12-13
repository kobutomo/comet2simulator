mod memory;
mod comet;

fn main() {
    let mut mem = memory::new();
    mem.write(12, 7000);
    let i = mem.read(12).unwrap();
    println!("{:08}", i)
}
